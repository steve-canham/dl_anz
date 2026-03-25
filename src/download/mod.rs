//mod file_models;
//mod processor;
pub mod data_access;
pub mod gen_helper;
//mod who_helper;

use crate::data_models::xl_data_models::*;
use crate::data_models::data_vecs::*;
use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use calamine::{open_workbook, Reader, Data, DataType, Xlsx, Range};
//use data_access::{update_who_study_mon, add_new_single_file_record, 
    //add_contents_record, store_who_summary};
//use file_models::WHOLine;
//use super::setup::config_reader::fetch_src_db_name;
//use std::fs;
//use std::io::BufReader;
//use std::fs::File;
//use csv::ReaderBuilder;
//use std::io::Write;
//use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
use log::info;

pub async fn setup_xl_tables(pool: &Pool<Postgres>) -> Result<(), AppError> {

    let sql = include_str!("../../sql/xl_tables.sql");

    sqlx::raw_sql(sql).execute(pool)
    .await
    .map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(())
}

pub async fn process_excel_file(file_path: &PathBuf, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let mut workbook: Xlsx<_> = open_workbook(file_path)
    .map_err(|_| AppError::CalError(format!("Cannot open excel workbook at {}", file_path.display())))?;
    
    let range = workbook.worksheet_range("SECONDARY ID")
    .map_err(|_| AppError::CalError("Cannot find sheet SECONDARY ID".to_string()))?;
    let _r1 = do_single_fields(&range, "secondary_ids", "sec_id", pool).await?;

    let range = workbook.worksheet_range("HEALTH CONDITION")
    .map_err(|_| AppError::CalError("Cannot find sheet HEALTH CONDITION".to_string()))?;
    let r2 = do_single_fields(&range, "health_conditions", "health_condition", pool).await?;

    let range = workbook.worksheet_range("INTERVENTION CODE")
    .map_err(|_| AppError::CalError("Cannot find sheet INTERVENTION CODE".to_string()))?;
    let _r3 = do_single_fields(&range, "intervention_codes", "intervention_code", pool).await?;

    

    Ok(r2)

}


async fn do_single_fields(range: &Range<Data>, table_name: &str, field_name: &str,  pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = SingleDataFields::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s) =  r[1].as_string() {
                let sid = s.trim()
                                .replace("\"", "").replace("'", "")
                                .replace("‘", "");
                let sid = sid.trim_start_matches(&['-', '“']);
                let low_sid = sid.to_lowercase();

                let valid = match table_name {
                    "secondary_ids" => valid_sec_id(&low_sid),
                    "health_conditions" => valid_condition(&low_sid),
                    "intervention_codes" => valid_int_code(&low_sid),
                    _ => false
                };

                if valid {
                    data_vecs.add(XLSingleDataField {
                        trial_id: tid,
                        data_field: Some(sid.to_string()),
                    });
                    added +=1;
                    n +=1;

                    if n == 250 {
                        data_vecs.store_data(table_name, field_name, pool).await?;
                        data_vecs = SingleDataFields::new(250);
                        n = 0;
                    }
                }
            }
        }

    }
        
    data_vecs.store_data(table_name, field_name, pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("{} {} records examined", table_name, examined);
    info!("{} {} records added", table_name, added);
    info!("");

    Ok(res)
}


fn valid_sec_id (low_sid: &String) -> bool {
    let mut validity = true;

    if low_sid.len() <= 2 || low_sid == "***" 
        || low_sid == "unknown"  {validity = false;}

    if low_sid.starts_with('n') {
        if low_sid.starts_with("nil")  || low_sid.starts_with("none")
        || low_sid.starts_with("not ") || low_sid.starts_with("no ") 
        || low_sid.starts_with("non") || low_sid.starts_with("no.") 
        || low_sid == "n/a" || low_sid == "n/s" || low_sid == "n.a." || low_sid == "na." 
        || low_sid == "ni known" || low_sid == "nik known" 
        || low_sid == "nihil" || low_sid.starts_with("new secondary id. please modify")
        {validity = false;}
    }

    if low_sid.starts_with('t') {
        if low_sid.starts_with("there ") || low_sid.starts_with("the trial ")
        || low_sid.starts_with("there's") || low_sid.starts_with("this trial does not") 
        || low_sid.starts_with("this study has") || low_sid.starts_with("trial has not") 
        {validity = false;}
    }

    validity

}


fn valid_condition (low_sid: &String) -> bool {
    let mut validity = true;

    if low_sid.starts_with('n') {
        if low_sid.starts_with("nil")  || low_sid.starts_with("none")
        || low_sid == "n/a" || low_sid == "n/s" || low_sid == "n.a." || low_sid == "na." 
        {validity = false;}
    }
    validity
}


fn valid_int_code (low_sid: &String) -> bool {

    low_sid != "none" && low_sid != "not applicable" 
}


/* 
pub async fn xrocess_files(file_path: &PathBuf, json_path: &PathBuf, dl_id:i32, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // Set up source file, csv reader, counters, hash table.
    
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    
    let mut file_res = DownloadResult::new();
    let mut source_tots: HashMap<i32, i32> = HashMap::new();
    info!("");
    info!("Processing file {:?}", file_path);

    for result in csv_rdr.deserialize() {

        file_res.num_checked +=1;
        if file_res.num_checked % 5000 == 0 {
            info!("{} records checked", file_res.num_checked);
        }

        let who_line: WHOLine = match result {
             Ok(w) => w,
             Err(e) => return Err(AppError::CsvError(e)),
        };
               
        let rec_summ = match processor::summarise_line(&who_line, file_res.num_checked)
        {
            Some(r) => r,
            None => continue,   // some sort of problem occured - should have been loggged
        };
        
        // Adjust running source totals (even if file not processed further)

        let source_id = rec_summ.source_id;
        source_tots.entry(source_id).and_modify(|n| *n += 1).or_insert(1);

        // assemble variables from summary record, allows them to be used 
        // later even if the whole record has already been moved for storage 

        let sid = rec_summ.sd_sid.clone();
        let date_of_rec = rec_summ.date_last_rev;
        let study_type = rec_summ.study_type;
        let study_status = rec_summ.study_status;
        let remote_url = rec_summ.remote_url.clone();
        let idents = rec_summ.secondary_ids.clone();
        let countries = rec_summ.country_list.clone();
        
        store_who_summary(rec_summ, pool).await?;             // add or update summary database record

        let mut full_path = PathBuf::from ("");
        let db_name = fetch_src_db_name()?;

        if source_id != 100120  && source_id != 100126 {           // file production not necessary for these sources
  
            match processor::process_line(who_line, source_id, &sid,         // get full version of WHO record
                study_type, study_status, &remote_url, idents, countries)
            {
                Some (rec) => {
                    let file_folder: PathBuf = [json_path, &PathBuf::from(&db_name)].iter().collect();
                    if !folder_exists(&file_folder) {
                        fs::create_dir_all(&file_folder)?;
                    }
                    let file_name = format!("{}.json", &sid);
                    full_path = [file_folder, PathBuf::from(&file_name)].iter().collect();
        
                    // Write the JSON string to a file - see if it is a new download, or an existing one.
                    
                    let json_string = to_string_pretty(&rec).unwrap();
                    let mut file = File::create(&full_path)?;
                    file.write_all(json_string.as_bytes())?;
                },

                None => continue,  // some sort of problem occured - should have been loggged
            }
        }

        // Update database and res accordingly.
        let added = update_who_study_mon(&db_name, &sid, &remote_url, dl_id,
                        &date_of_rec, &full_path, pool).await?;

        file_res.num_downloaded +=1;
        if added {
            file_res.num_added +=1;
        } 
    }

    info!("{} records checked in total for this file", file_res.num_checked);
    info!("---------------------------------------------------");

    // Update database with single file details and 
    // return the aggregate figures in the res struct ... 

    add_new_single_file_record(dl_id, file_path, &file_res, pool).await?;
    add_contents_record(file_path, &mut source_tots, pool).await?;

    Ok(file_res)

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
*/