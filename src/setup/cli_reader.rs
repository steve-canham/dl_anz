use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub importing: bool,
    pub transforming: bool,
    pub coding: bool,
    pub source_file: String,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    let mut i_flag = parse_result.get_flag("i_flag");
    let mut t_flag = parse_result.get_flag("t_flag");
    let mut c_flag = parse_result.get_flag("c_flag");
    let a_flag = parse_result.get_flag("a_flag");

    // Source file usually part of CLI commands but can be provided by config file

    let source_file = parse_result.get_one::<String>("file").unwrap();

    // If no flag do the import as the default.

    if !i_flag  && !t_flag && !c_flag  {
        i_flag = true;
    }

    if a_flag {
        i_flag = true;
        t_flag = true;
        c_flag = true;
    }

    Ok(CliPars {
        importing: i_flag,
        transforming: t_flag,
        coding: c_flag,
        source_file: source_file.clone(),
    }) 
}


pub fn config_file_exists()-> bool {
    let config_path = PathBuf::from("./app_config.toml");
    let res = match config_path.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {

    command!()
        .about("Imports data from excel file and imports it into the database xl schema")
        .arg(
            Arg::new("i_flag")
           .short('i')
           .long("import")
           .required(false)
           .help("A flag signifying data to be imported from an excel file")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("file")
           .short('f')
           .long("file")
           .required(false)
           .help("A string with the source file name")
           .default_value("")    // Note default value of ""
        )
        .arg(
            Arg::new("t_flag")
           .short('t')
           .long("transform")
           .required(false)
           .help("A flag signifying xl data to be transformed to mdr schema and put into sd then ad tables")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("c_flag")
           .short('c')
           .long("coding")
           .required(false)
           .help("A flag signifying ad data should be coded where appropriate")
           .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("a_flag")
           .short('a')
           .long("do_all")
           .required(false)
           .help("A flag signifying i, t and c flags should be applied, in that order")
           .action(clap::ArgAction::SetTrue)
        )
    .try_get_matches_from(args)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cli_no_explicit_params() {
        let target = "dummy target";
        let args: Vec<&str> = vec![target];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, false);
        assert_eq!(res.coding, false);
        assert_eq!(res.source_file, "");
    }  

    #[test]
    fn check_cli_with_t_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, false);
        assert_eq!(res.transforming, true);
        assert_eq!(res.coding, false);
        assert_eq!(res.source_file, "");
    }


    #[test]
    fn check_cli_with_c_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-c"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, false);
        assert_eq!(res.transforming, false);
        assert_eq!(res.coding, true);
        assert_eq!(res.source_file, "");
    }

    #[test]
    fn check_cli_with_import_and_source() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i", "-f", "dummy file.xlsx"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, false);
        assert_eq!(res.coding, false);
        assert_eq!(res.source_file, "dummy file.xlsx");
    }

    #[test]
    fn check_cli_with_all_flags() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i", "-t", "-c", "-f", "dummy file.xlsx"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, true);
        assert_eq!(res.coding, true);
        assert_eq!(res.source_file, "dummy file.xlsx");
    }

    #[test]
    fn check_cli_with_a_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-a", "-f", "dummy file.xlsx"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.importing, true);
        assert_eq!(res.transforming, true);
        assert_eq!(res.coding, true);
        assert_eq!(res.source_file, "dummy file.xlsx");
    }
   
}

