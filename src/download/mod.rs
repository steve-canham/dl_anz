pub mod data_access;
pub mod gen_helper;
pub mod dl_helper;

use crate::data_models::xl_data_models::*;
use crate::data_models::data_vecs::*;

use dl_helper::{get_as_string_option, get_date_as_string_option, get_datetime_as_string_option};
use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use calamine::{open_workbook, Reader, Data, DataType, Xlsx, Range};
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

    
    let range = workbook.worksheet_range("TRIAL")
    .map_err(|_| AppError::CalError("Cannot find sheet TRIAL".to_string()))?;
    let r = do_trials(&range, pool).await?;

    let range = workbook.worksheet_range("SECONDARY ID")
    .map_err(|_| AppError::CalError("Cannot find sheet SECONDARY ID".to_string()))?;
    do_single_fields(&range, "secondary_ids", "sec_id", pool).await?;

    let range = workbook.worksheet_range("HEALTH CONDITION")
    .map_err(|_| AppError::CalError("Cannot find sheet HEALTH CONDITION".to_string()))?;
    do_single_fields(&range, "health_conditions", "health_condition", pool).await?;

    let range = workbook.worksheet_range("INTERVENTION CODE")
    .map_err(|_| AppError::CalError("Cannot find sheet INTERVENTION CODE".to_string()))?;
    do_single_fields(&range, "intervention_codes", "intervention_code", pool).await?;

    
    let range = workbook.worksheet_range("HOSPITAL")
    .map_err(|_| AppError::CalError("Cannot find sheet HOSPITAL".to_string()))?;
    do_hospitals (&range, pool).await?;


    let range = workbook.worksheet_range("CONDITION  CODE")
    .map_err(|_| AppError::CalError("Cannot find sheet CONDITION  CODE".to_string()))?;
    do_double_fields(&range, "condition_codes", "condition_category", "condition_code", pool).await?;

    let range = workbook.worksheet_range("COUNTRY OUTSIDE AUSTRALIA")
    .map_err(|_| AppError::CalError("Cannot find sheet COUNTRY OUTSIDE AUSTRALIA".to_string()))?;
    do_double_fields(&range, "other_countries", "country", "state", pool).await?;


    let range = workbook.worksheet_range("FUNDING SOURCE")
    .map_err(|_| AppError::CalError("Cannot find sheet FUNDING SOURCE".to_string()))?;
    do_type_name_countries(&range, "funding_sources", pool).await?;

    let range = workbook.worksheet_range("SECONDARY SPONSOR")
    .map_err(|_| AppError::CalError("Cannot find sheet SECONDARY SPONSOR".to_string()))?;
    do_type_name_countries(&range, "secondary_sponsors", pool).await?;

    let range = workbook.worksheet_range("OTHER COLLABORATOR")
    .map_err(|_| AppError::CalError("Cannot find sheet OTHER COLLABORATOR".to_string()))?;
    do_type_name_countries(&range, "other_collaborators", pool).await?;


    let range = workbook.worksheet_range("PRIMARY OUTCOME")
    .map_err(|_| AppError::CalError("Cannot find sheet PRIMARY OUTCOME".to_string()))?;
    do_outcomes(&range, "primary_outcomes", pool).await?;

    let range = workbook.worksheet_range("SECONDARY OUTCOME")
    .map_err(|_| AppError::CalError("Cannot find sheet SECONDARY OUTCOME".to_string()))?;
    do_outcomes(&range, "secondary_outcomes", pool).await?;

    
    let range = workbook.worksheet_range("ETHICS COMMITTEE")
    .map_err(|_| AppError::CalError("Cannot find sheet ETHICS COMMITTEE".to_string()))?;
    do_ethics_committees(&range, pool).await?;

    let range = workbook.worksheet_range("CONTACTS")
    .map_err(|_| AppError::CalError("Cannot find sheet CONTACTS".to_string()))?;
    do_contacts(&range, pool).await?;

    let range = workbook.worksheet_range("DATA SHARING STATEMENT")
    .map_err(|_| AppError::CalError("Cannot find sheet DATA SHARING STATEMENT".to_string()))?;
    do_dss(&range, pool).await?;
    
    let range = workbook.worksheet_range("SUPPORTING DOCUMENTS")
    .map_err(|_| AppError::CalError("Cannot find sheet SUPPORTING DOCUMENTS".to_string()))?;
    do_supporting_docs(&range, pool).await?;

    let range = workbook.worksheet_range("STUDY RESULTS")
    .map_err(|_| AppError::CalError("Cannot find sheet STUDY RESULTS".to_string()))?;
    do_study_results(&range, pool).await?;

    let range = workbook.worksheet_range("EXTERNAL PUBLICATIONS")
    .map_err(|_| AppError::CalError("Cannot find sheet EXTERNAL PUBLICATIONS".to_string()))?;
    do_external_pubs(&range, pool).await?;

    Ok(r)

}


async fn do_trials(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let mut examined = 0;
    let mut added = 0;

    let mut trial_vecs = Trials::new(250);
    let mut lc_vecs = StudyLifeCycles::new(250);
    let mut sf_vecs = StudyFeaturess::new(250);
    let mut ps_vecs = Participantss::new(250);

    let mut n = 0;
   
    for (row_num, r) in range.rows().enumerate() {
        if row_num != 0 {

            examined +=1;

            if let Some(id) =  r[0].as_i64() { 

                let tid = id as i32;

                if let Some(sid) =  r[1].as_string() {

                    //let sub_date = get_datetime_as_string_option(r[2].clone());   //C
                    //let app_date = get_datetime_as_string_option(r[3].clone());
                    //let title = get_as_string_option(r[4].clone());
                    //let sci_title = get_as_string_option(r[5].clone());
                    //let utn = get_as_string_option(r[6].clone());      //G
                    let acronym = get_as_string_option(r[7].clone());
                    let linked_study = get_as_string_option(r[8].clone());
                    let intervents = get_as_string_option(r[9].clone());
                    let comp = get_as_string_option(r[10].clone());
                    let control = get_as_string_option(r[11].clone());
                    let inc_crit = get_as_string_option(r[12].clone());  //M
                    let min_age = get_as_string_option(r[13].clone());
                    let min_age_type = get_as_string_option(r[14].clone());
                    let max_age = get_as_string_option(r[15].clone());
                    let max_age_type = get_as_string_option(r[16].clone());
                    let gender = get_as_string_option(r[17].clone());    // R
                    let volunteers = get_as_string_option(r[18].clone()); 
                    let exc_crit = get_as_string_option(r[19].clone());
                    let study_type = get_as_string_option(r[20].clone());
                    let purpose = get_as_string_option(r[21].clone());
                    let allocation = get_as_string_option(r[22].clone()); 
                    let concealment = get_as_string_option(r[23].clone());
                    let sequence = get_as_string_option(r[24].clone());
                    let masking = get_as_string_option(r[25].clone()); //Z
                    let assignment = get_as_string_option(r[26].clone());
                    let other_feats = get_as_string_option(r[27].clone());
                    let endpoint = get_as_string_option(r[28].clone());
                    let phase = get_as_string_option(r[29].clone());
                    let stat_methods = get_as_string_option(r[30].clone()); // AE
                    let msk_parts = get_as_string_option(r[31].clone());
                    let msk_clins = get_as_string_option(r[32].clone());
                    let msk_out = get_as_string_option(r[33].clone());
                    let msk_ana = get_as_string_option(r[34].clone());
                    let pt_reg = get_as_string_option(r[35].clone());  // AJ
                    let reg_fu = get_as_string_option(r[36].clone());
                    let reg_fu_type = get_as_string_option(r[37].clone());
                    let obs_purp = get_as_string_option(r[38].clone());
                    let obs_dura = get_as_string_option(r[39].clone());
                    let obs_selec = get_as_string_option(r[40].clone());
                    let obs_timing = get_as_string_option(r[41].clone()); // AP
                    let antic_sd = get_date_as_string_option(r[42].clone());
                    let actual_sd = get_date_as_string_option(r[43].clone());
                    let antic_ed = get_date_as_string_option(r[44].clone());
                    let actual_ed = get_date_as_string_option(r[45].clone());
                    let target_ss = get_as_string_option(r[46].clone());  // AU
                    let final_ss = get_as_string_option(r[47].clone());
                    let current_ss = get_as_string_option(r[48].clone());
                    let antic_lvd = get_date_as_string_option(r[49].clone());
                    let actual_lvd = get_date_as_string_option(r[50].clone());
                    let rec_status = get_as_string_option(r[51].clone());  // AZ

                    let r_len= r.len();

                    let data_ana = if r_len > 52 {get_as_string_option(r[52].clone())} else {None};
                    let wdrawn_reas = if r_len > 53 {get_as_string_option(r[53].clone())} else {None};
                    let wdrawn_reas_oth = if r_len > 54 {get_as_string_option(r[54].clone())} else {None};
                    let rec_country = if r_len > 55 {get_as_string_option(r[55].clone())} else {None};
                    let rec_state = if r_len > 56 {get_as_string_option(r[56].clone())} else {None};  // BE
                    let pri_spons_type = if r_len > 57 {get_as_string_option(r[57].clone())} else {None};
                    let pri_spons_name = if r_len > 58 {get_as_string_option(r[58].clone())} else {None};
                    let pri_spon_count = if r_len > 59 {get_as_string_option(r[59].clone())} else {None};
                    let ethics_status = if r_len > 60 {get_as_string_option(r[60].clone())} else {None};
                    let brief_summ = if r_len > 61 {get_as_string_option(r[61].clone())} else {None};
                    let website = if r_len > 62 {get_as_string_option(r[62].clone())} else {None};  //BK
                    let publication = if r_len > 63 {get_as_string_option(r[63].clone())} else {None};
                    let pub_notes = if r_len > 64 {get_as_string_option(r[64].clone())} else {None};  // BM

                
                    trial_vecs.add(XLTrial {
                        trial_id: tid, 
                        actrn_id: sid.clone(), 
                        submit_date: get_datetime_as_string_option(r[2].clone()), 
                        approval_date: get_datetime_as_string_option(r[3].clone()), 
                        study_title: get_as_string_option(r[4].clone()), 
                        scientific_title: get_as_string_option(r[5].clone()), 
                        utn: get_as_string_option(r[6].clone()), 
                        trial_acronym: acronym, 
                        linked_study: linked_study, 
                        study_type: study_type, 
                        patient_registry: pt_reg, 
                        registry_followup: reg_fu, 
                        registry_followup_type: reg_fu_type, 
                        primary_sponsor_type: pri_spons_type, 
                        primary_sponsor_name: pri_spons_name, 
                        primary_sponsor_country: pri_spon_count, 
                        ethics_status: ethics_status, 
                        brief_summary: brief_summ, 
                        trial_website: website, 
                        publication: publication, 
                        public_notes: pub_notes, 
                    });

                    lc_vecs.add(XLStudyLifeCycle { 
                        trial_id: tid, 
                        actrn_id: sid.clone(), 
                        antic_start_date: antic_sd, 
                        actual_start_date: actual_sd, 
                        antic_end_date: antic_ed, 
                        actual_end_date: actual_ed, 
                        antic_last_visit_date: antic_lvd, 
                        actual_last_visit_date: actual_lvd, 
                        recruitment_status: rec_status, 
                        data_analysis: data_ana, 
                        withdrawn_reason: wdrawn_reas, 
                        withdrawn_reason_other: wdrawn_reas_oth, 
                        recruitment_country: rec_country, 
                        recruitmenbt_state: rec_state,
                    });

                    sf_vecs.add(XLStudyFeatures { 
                        trial_id: tid, 
                        actrn_id: sid.clone(), 
                        interventions: intervents, 
                        comparator: comp, 
                        control: control, 
                        purpose: purpose, 
                        allocation: allocation, 
                        concealment: concealment, 
                        sequencing: sequence, 
                        masking: masking, 
                        assignment: assignment, 
                        other_design_features: other_feats, 
                        endpoint: endpoint, 
                        phase: phase, 
                        stat_methods: stat_methods, 
                        masking_participants: msk_parts, 
                        masking_clinicians: msk_clins, 
                        masking_assessors: msk_out, 
                        masking_analysts: msk_ana, 
                        obs_purpose: obs_purp, 
                        obs_duration: obs_dura, 
                        obs_selection: obs_selec, 
                        obs_timing: obs_timing,
                    });

                    ps_vecs.add(XLParticipants { 
                        trial_id: tid, 
                        actrn_id: sid.clone(),  
                        inclusion_criteria: inc_crit, 
                        min_age: min_age, 
                        min_age_type: min_age_type, 
                        max_age: max_age, 
                        max_age_type: max_age_type, 
                        gender: gender, 
                        healthy_volunteers: volunteers, 
                        exclusion_criteria: exc_crit, 
                        target_sample_size: target_ss, 
                        final_sample_size: final_ss, 
                        current_sample_size: current_ss,
                    });

                    added +=1;
                    n +=1;

                    if n == 250 {

                        trial_vecs.store_data(pool).await?;
                        lc_vecs.store_data(pool).await?;
                        sf_vecs.store_data(pool).await?;
                        ps_vecs.store_data(pool).await?;

                        trial_vecs = Trials::new(250);
                        lc_vecs = StudyLifeCycles::new(250);
                        sf_vecs = StudyFeaturess::new(250);
                        ps_vecs = Participantss::new(250);

                        n = 0;
                        
                    }
                }
            }
        }
    }


    trial_vecs.store_data(pool).await?;
    lc_vecs.store_data(pool).await?;
    sf_vecs.store_data(pool).await?;
    ps_vecs.store_data(pool).await?;


    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("trials {} records examined", examined);
    info!("trials {} records added", added);
    info!("");

    Ok(res)

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

                let sid = match table_name {
                    "secondary_ids" => dl_helper::trim_sec_id(s.trim()),
                    "health_conditions" => dl_helper::trim_condition(s.trim()),
                    "intervention_codes" => s,
                    "hospitals" => s.replace("'", "’"),
                    _ => s
                };

                let low_sid = sid.to_lowercase();

                let valid = match table_name {
                    "secondary_ids" => dl_helper::valid_sec_id(&low_sid),
                    "health_conditions" => dl_helper::valid_condition(&low_sid),
                    "intervention_codes" => dl_helper::valid_int_code(&low_sid),
                    "hospitals" => true,
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


async fn do_hospitals(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = DoubleDataFields::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s) =  r[1].as_string() {

                let sid = s.replace("'", "’");

                // split final hyphenated group into s2

                let sid_bits: Vec<&str> = sid.split(" - ").collect();

                if sid_bits.len() == 1 {
                    data_vecs.add(XLDoubleDataField {
                        trial_id: tid,
                        data_field1: Some(sid.to_string()),
                        data_field2: None,
                    });
                }
                else if sid_bits.len() == 2 {
                    data_vecs.add(XLDoubleDataField {
                        trial_id: tid,
                        data_field1: Some(sid_bits[0].to_string()),
                        data_field2: Some(sid_bits[1].to_string()),
                    });
                }
                else {
                    let last_bit = sid_bits[sid_bits.len()-1].to_string();
                    let first_bit = sid_bits[..sid_bits.len() -2].join(" - ");
                    data_vecs.add(XLDoubleDataField {
                        trial_id: tid,
                        data_field1: Some(first_bit),
                        data_field2: Some(last_bit),
                    });
                }

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data("hospitals", "hospital", "location", pool).await?;
                    data_vecs = DoubleDataFields::new(250);
                    n = 0;
                }

            }
        }

    }
        
    data_vecs.store_data("hospitals", "hospital", "location", pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("hospitals {} records examined", examined);
    info!("hospitals {} records added", added);
    info!("");

    Ok(res)
}


async fn do_double_fields(range: &Range<Data>, table_name: &str, field_name1: &str, field_name2: &str, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = DoubleDataFields::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                 let sid1 = match table_name {
                    "condition_codes" => s1,
                    "other_countries" => s1.trim().to_string(),
                    _ => s1
                };

                let sid2: Option<String>;
                if let Some(s2) =  r[2].as_string() {
                    sid2 = match table_name {
                        "condition_codes" => Some(s2),
                        "other_countries" => Some(s2.trim().to_string()),
                        _ => Some(s2),
                    };
                } else {
                    sid2 = match table_name {
                        "condition_codes" => Some("Not Specified".to_string()),
                        "other_countries" => None,
                        _ => None,
                    };
                }

                // all records classed as valid
                
                data_vecs.add(XLDoubleDataField {
                    trial_id: tid,
                    data_field1: Some(sid1.to_string()),
                    data_field2: sid2,
                });
                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(table_name, field_name1, field_name2, pool).await?;
                    data_vecs = DoubleDataFields::new(250);
                    n = 0;
                }
                
            }
        }

    }
        
    data_vecs.store_data(table_name, field_name1, field_name2, pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("{} {} records examined", table_name, examined);
    info!("{} {} records added", table_name, added);
    info!("");

    Ok(res)
}


async fn do_type_name_countries(range: &Range<Data>, table_name: &str, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = TypeNameCountries::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                 if let Some(s2) =  r[2].as_string() {

                    let sid2: String = match table_name {
                        "funding_sources" => dl_helper::trim_funding_source(s2.trim()),
                        "secondary_sponsors" => dl_helper::trim_sec_sponsors(s2.trim()),
                        "other_collaborators" => dl_helper::trim_other_collabs(s2.trim()),
                        _ => s2
                    };

                    let low_sid2 = sid2.to_lowercase();

                    let valid = match table_name {
                        "funding_sources" => dl_helper::valid_funding_source(&low_sid2),
                        "secondary_sponsors" => s1 != "None",
                        "other_collaborators" => true,
                        _ => false
                    };

                    if valid {

                        let sid3 = match r.len() > 3 {
                            true => r[3].as_string(),
                            false => None,
                        };
                                                                     
                        data_vecs.add(XLTypeNameCountry {
                            trial_id: tid,
                            entity_type: Some(s1),
                            name: Some(sid2),
                            country: sid3
                        });

                        added +=1;
                        n +=1;

                        if n == 250 {
                            data_vecs.store_data(table_name, pool).await?;
                            data_vecs = TypeNameCountries::new(250);
                            n = 0;
                        }
                    }
                
                }
            }

        }
    }
            
    data_vecs.store_data(table_name, pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("{} {} records examined", table_name, examined);
    info!("{} {} records added", table_name, added);
    info!("");

    Ok(res)
    
}


async fn do_outcomes(range: &Range<Data>, table_name: &str, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = Outcomes::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };
                       
                data_vecs.add(XLOutcome {
                    trial_id: tid,
                    outcome: Some(s1),
                    outcome_assessment: sid2,
                    timepoint: sid3
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(table_name, pool).await?;
                    data_vecs = Outcomes::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(table_name, pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("{} {} records examined", table_name, examined);
    info!("{} {} records added", table_name, added);
    info!("");

    Ok(res)
    
}


async fn do_ethics_committees(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = EthicsCommittees::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };

                let sid4 = match r.len() > 4 {
                    true => r[4].as_string(),
                    false => None,
                };

                let sid5 = match r.len() > 5 {
                    true => r[5].as_string(),
                    false => None,
                };

                let sid6 = match r.len() > 6 {
                    true => r[6].as_string(),
                    false => None,
                };
                     
                data_vecs.add(XLEthicsCommittee {
                    trial_id: tid,
                    name: Some(s1),
                    address: sid2,
                    country: sid3,
                    submit_date: sid4, 
                    approval_date: sid5,
                    hrec_approval_id: sid6,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = EthicsCommittees::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("ethics_committees {} records examined", examined);
    info!("ethics_committees {} records added", added);
    info!("");

    Ok(res)
    
}


async fn do_contacts(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = Contacts::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {true => r[2].as_string(), false => None,};
                let sid3 = match r.len() > 3 {true => r[3].as_string(), false => None,};
                let sid4 = match r.len() > 4 {true => r[4].as_string(), false => None,};
                let sid5 = match r.len() > 5 {true => r[5].as_string(), false => None,};
                let sid6 = match r.len() > 6 {true => r[6].as_string(), false => None,};
                let sid7 = match r.len() > 7 {true => r[7].as_string(), false => None,};
                let sid8 = match r.len() > 8 {true => r[8].as_string(), false => None,};
                       
                data_vecs.add(XLContact {
                    trial_id: tid,
                    contact_type: Some(s1),
                    title: sid2,
                    name: sid3,
                    address: sid4,
                    country: sid5,
                    phone: sid6,
                    fax: sid7,
                    email: sid8,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = Contacts::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("contacts {} records examined", examined);
    info!("contacts {} records added", added);
    info!("");

    Ok(res)
    
}


async fn do_dss(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = DSSs::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };

                let sid4 = match r.len() > 4 {
                    true => r[4].as_string(),
                    false => None,
                };

                let sid5 = match r.len() > 5 {
                    true => r[5].as_string(),
                    false => None,
                };

                let sid6 = match r.len() > 6 {
                    true => r[6].as_string(),
                    false => None,
                };

                let sid7 = match r.len() > 7 {
                    true => r[7].as_string(),
                    false => None,
                };

                let sid8 = match r.len() > 8 {
                    true => r[8].as_string(),
                    false => None,
                };

                let sid9 = match r.len() > 9 {
                    true => r[9].as_string(),
                    false => None,
                };
                       
                data_vecs.add(XLDSS {
                    trial_id: tid,
                    ipd_availability: Some(s1),
                    available_to_whom: sid2,
                    availability_conditions: sid3,
                    data_to_be_shared: sid4,
                    for_what_analyses_types: sid5,
                    timeframe_from: sid6,
                    timeframe_to: sid7,
                    mechanism: sid8,
                    extra_considerations: sid9,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = DSSs::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("dss {} records examined", examined);
    info!("dss {} records added", added);
    info!("");

    Ok(res)
    
}


async fn do_supporting_docs(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = SuppDocs::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };

                let sid4 = match r.len() > 4 {
                    true => r[4].as_string(),
                    false => None,
                };

                let sid5 = match r.len() > 5 {
                    true => r[5].as_string(),
                    false => None,
                };

                let sid6 = match r.len() > 6 {
                    true => r[6].as_string(),
                    false => None,
                };              
                       
                data_vecs.add(XLSuppDoc {
                    trial_id: tid,
                    doc_type: Some(s1),
                    citation: sid2,
                    link: sid3,
                    email: sid4,
                    details: sid5,
                    attachment: sid6,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = SuppDocs::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("supporting_docs {} records examined", examined);
    info!("supporting_docs {} records added", added);
    info!("");

    Ok(res)
    
}


async fn do_study_results(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = StudyResults::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };

                let sid4 = match r.len() > 4 {
                    true => r[4].as_string(),
                    false => None,
                };

                let sid5 = match r.len() > 5 {
                    true => r[5].as_string(),
                    false => None,
                };
                      
                data_vecs.add(XLStudyResult {
                    trial_id: tid,
                    results_type: Some(s1),
                    is_peer_reviewed: sid2,
                    doi: sid3,
                    citations_or_details: sid4,
                    attachment: sid5,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = StudyResults::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("study_results {} records examined", examined);
    info!("study_results {} records added", added);
    info!("");

    Ok(res)
    
}


async fn do_external_pubs(range: &Range<Data>, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    let data_rows: Vec<Vec<Data>> = range
            .rows().skip(1)    // jump over header
            .map(|row| row.iter()
                    .map(|c| c.clone()).collect())  // each row as an array of data
            .collect();

    let mut examined = 0;
    let mut added = 0;
    let mut data_vecs = ExternalPublications::new(250);
    let mut n = 0;

    for r in data_rows {

        examined += 1;

        if let Some(id) =  r[0].as_i64() {     // valid id

            let tid = id as i32;

            if let Some(s1) =  r[1].as_string() {

                // Assume all valid and that text does not need correction

                let sid2 = match r.len() > 2 {
                    true => r[2].as_string(),
                    false => None,
                };

                let sid3 = match r.len() > 3 {
                    true => r[3].as_string(),
                    false => None,
                };

                let sid4 = match r.len() > 4 {
                    true => r[4].as_string(),
                    false => None,
                };
  
                data_vecs.add(XLExternalPublication {
                    trial_id: tid,
                    source: Some(s1),
                    doi: sid2,
                    title: sid3,
                    year_of_publication: sid4,
                });

                added +=1;
                n +=1;

                if n == 250 {
                    data_vecs.store_data(pool).await?;
                    data_vecs = ExternalPublications::new(250);
                    n = 0;
                }
            }
        }
    }
            
    data_vecs.store_data(pool).await?;

    let mut res = DownloadResult::new();
    res.num_checked = examined;
    res.num_downloaded = added;
    res.num_added = added;

    info!("external_publications {} records examined", examined);
    info!("external_publications {} records added", added);
    info!("");

    Ok(res)
    
}


