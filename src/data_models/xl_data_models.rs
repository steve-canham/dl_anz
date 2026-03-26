
pub struct XLTrial {
    pub trial_id: i32, 
    pub actrn_id: String, 
    pub submit_date: Option<String>, 
    pub approval_date: Option<String>, 
    pub study_title: Option<String>, 
    pub scientific_title: Option<String>, 
    pub utn: Option<String>, 
    pub trial_acronym: Option<String>, 
    pub linked_study: Option<String>, 
    pub study_type: Option<String>, 
    pub patient_registry: Option<String>, 
    pub registry_followup: Option<String>, 
    pub registry_followup_type: Option<String>, 
    pub primary_sponsor_type: Option<String>, 
    pub primary_sponsor_name: Option<String>, 
    pub primary_sponsor_country: Option<String>, 
    pub ethics_status: Option<String>, 
    pub brief_summary: Option<String>, 
    pub trial_website: Option<String>, 
    pub publication: Option<String>, 
    pub public_notes: Option<String>, 
}


pub struct XLStudyLifeCycle {
    pub trial_id: i32, 
    pub actrn_id: String, 
    pub antic_start_date: Option<String>, 
    pub actual_start_date: Option<String>, 
    pub antic_end_date: Option<String>, 
    pub actual_end_date: Option<String>, 
    pub antic_last_visit_date: Option<String>, 
    pub actual_last_visit_date: Option<String>, 
    pub recruitment_status: Option<String>, 
    pub data_analysis: Option<String>, 
    pub withdrawn_reason: Option<String>, 
    pub withdrawn_reason_other: Option<String>, 
    pub recruitment_country: Option<String>, 
    pub recruitmenbt_state: Option<String>, 
}


pub struct XLStudyFeatures {
    pub trial_id: i32, 
    pub actrn_id: String, 
    pub interventions: Option<String>, 
    pub comparator: Option<String>, 
    pub control: Option<String>, 
    pub purpose: Option<String>, 
    pub allocation: Option<String>, 
    pub concealment: Option<String>, 
    pub sequencing: Option<String>, 
    pub masking: Option<String>, 
    pub assignment: Option<String>, 
    pub other_design_features: Option<String>, 
    pub endpoint: Option<String>, 
    pub phase: Option<String>, 
    pub stat_methods: Option<String>, 
    pub masking_participants: Option<String>, 
    pub masking_clinicians: Option<String>, 
    pub masking_assessors: Option<String>, 
    pub masking_analysts: Option<String>, 
    pub obs_purpose: Option<String>, 
    pub obs_duration: Option<String>, 
    pub obs_selection: Option<String>, 
    pub obs_timing: Option<String>, 
}


pub struct XLParticipants {
    pub trial_id: i32, 
    pub actrn_id: String, 
    pub inclusion_criteria: Option<String>, 
    pub min_age: Option<String>, 
    pub min_age_type: Option<String>, 
    pub max_age: Option<String>, 
    pub max_age_type: Option<String>, 
    pub gender: Option<String>, 
    pub healthy_volunteers: Option<String>, 
    pub exclusion_criteria: Option<String>, 
    pub target_sample_size: Option<String>, 
    pub final_sample_size: Option<String>, 
    pub current_sample_size: Option<String>, 
}


// Use for secondary_ids, health_conditions, intervention_codes, hospitals

pub struct XLSingleDataField {
    pub trial_id: i32, 
    pub data_field: Option<String>, 
}

// Use for condition_codes, other_countries

pub struct XLDoubleDataField {
    pub trial_id: i32, 
    pub data_field1: Option<String>, 
    pub data_field2: Option<String>, 
}

// Use for primary_outcomes, secondary_outcomes

pub struct XLOutcome {
    pub trial_id: i32, 
    pub outcome: Option<String>, 
    pub outcome_assessment: Option<String>, 
    pub timepoint: Option<String>, 
}

// Use for funding_sources, secondary_sponsors, other_collaborators

pub struct XLTypeNameCountry {
    pub trial_id: i32, 
    pub entity_type: Option<String>, 
    pub name: Option<String>, 
    pub country: Option<String>, 
}

pub struct XLEthicsCommittee {
    pub trial_id: i32, 
    pub name: Option<String>, 
    pub address: Option<String>, 
    pub country: Option<String>, 
    pub submit_date: Option<String>, 
    pub approval_date: Option<String>, 
    pub hrec_approval_id: Option<String>, 

}

pub struct XLContact {
    pub trial_id: i32, 
    pub contact_type: Option<String>, 
    pub title: Option<String>, 
    pub name: Option<String>, 
    pub address: Option<String>, 
    pub country: Option<String>, 
    pub phone: Option<String>, 
    pub fax: Option<String>, 
    pub email: Option<String>, 
}


pub struct XLDSS {
    pub trial_id: i32, 
    pub ipd_availability: Option<String>, 
    pub available_to_whom: Option<String>, 
    pub availability_conditions: Option<String>, 
    pub data_to_be_shared: Option<String>, 
    pub for_what_analyses_types: Option<String>, 
    pub timeframe_from: Option<String>, 
    pub timeframe_to: Option<String>, 
    pub mechanism: Option<String>, 
    pub extra_considerations: Option<String>, 
}


pub struct XLSuppDoc {
    pub trial_id: i32, 
    pub doc_type: Option<String>, 
    pub citation: Option<String>, 
    pub link: Option<String>, 
    pub email: Option<String>, 
    pub details: Option<String>, 
    pub attachment: Option<String>, 
}


pub struct XLStudyResult {
    pub trial_id: i32, 
    pub results_type: Option<String>, 
    pub is_peer_reviewed: Option<String>, 
    pub doi: Option<String>, 
    pub citations_or_details: Option<String>, 
    pub attachment: Option<String>, 
}


pub struct XLExternalPublication {
    pub trial_id: i32, 
    pub source: Option<String>, 
    pub doi: Option<String>, 
    pub title: Option<String>, 
    pub year_of_publication: Option<String>, 
}