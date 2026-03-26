


pub fn trim_sec_id (s: &str) -> String {
    let sid = s.trim_start_matches(&['-', '“']);
    sid.replace("\"", "").replace("'", "").replace("‘", "")
}

pub fn valid_sec_id (low_sid: &String) -> bool {
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

pub fn trim_condition (s: &str) -> String {
    let sid = s.trim_start_matches(&['-', '\'']);
    let sid2 = sid.trim_end_matches(&[',', '.', '\'']);
    let sid3 = sid2.replace("'s", "’s");
    sid3.replace("\"", "").replace("‘", "")
}


pub fn valid_condition (low_sid: &String) -> bool {
    let mut validity = true;

    if low_sid.starts_with('n') {
        if low_sid.starts_with("nil")  || low_sid.starts_with("none")
        || low_sid == "n/a" || low_sid == "n/s" || low_sid == "n.a." || low_sid == "na." 
        {validity = false;}
    }
    validity
}

pub fn valid_int_code (low_sid: &String) -> bool {

    low_sid != "none" && low_sid != "not applicable" 
}


pub fn trim_funding_source (s: &str) -> String {
    let sid = s.trim_start_matches('.').trim();
    let sid2 = sid.replace("'s", "’s");
    sid2.replace("\"", "")
}


pub fn trim_sec_sponsors (s: &str) -> String {
    let sid = s.replace(".", " ");
    let sid2 = sid.replace("  ", " ");
    let sid3 = sid2.replace("Associate ", "Assoc ").replace("Professor", "Prof");
    let sid4 = sid3.replace("A/Prof", "Assoc Prof").replace("A Prof", "Assoc Prof");
    sid4.replace("A/P ", "Assoc Prof ").replace("A/Pr", "Assoc Prof")
}


pub fn trim_other_collabs (s: &str) -> String {
    let sid = s.replace(".", " ");
    let sid2 = sid.replace("  ", " ");
    let sid3 = sid2.replace("Associate ", "Assoc ").replace("Professor", "Prof");
    let sid4 = sid3.replace("A/Prof", "Assoc Prof").replace("A Prof", "Assoc Prof");
    sid4.replace("A/P ", "Assoc Prof ").replace("A/Pr", "Assoc Prof")
}


pub fn valid_funding_source (low_sid: &String) -> bool {
    let mut validity = true;

    if low_sid.len() <= 2 
        || low_sid == "unfunded"  {validity = false;}

    if low_sid.starts_with('n') {
        if low_sid.starts_with("nil")  || low_sid.starts_with("none")
        || low_sid.starts_with("not ") || low_sid.starts_with("no ") 
        || low_sid.starts_with("non ") 
        || low_sid.starts_with("n/a") ||  low_sid.starts_with("na")
        {validity = false;}
    }

    if low_sid.starts_with('t') {
        if low_sid.starts_with("there ") || low_sid.starts_with("this is ")
        || low_sid.starts_with("the study is ") || low_sid.starts_with("the project ") 
        {validity = false;}
    }
    validity
}
