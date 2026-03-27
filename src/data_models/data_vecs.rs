use crate::data_models::xl_data_models::*;
use crate::AppError;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};


pub struct Trials{
    pub trial_ids: Vec<i32>, 
    pub actrn_ids: Vec<String>, 
    pub submit_dates: Vec<Option<String>>, 
    pub approval_dates: Vec<Option<String>>, 
    pub study_titles: Vec<Option<String>>, 
    pub scientific_titles: Vec<Option<String>>, 
    pub utns: Vec<Option<String>>, 
    pub trial_acronyms: Vec<Option<String>>,  
    pub linked_studies: Vec<Option<String>>, 
    pub study_types: Vec<Option<String>>, 
    pub patient_registrys: Vec<Option<String>>, 
    pub registry_followups:Vec<Option<String>>, 
    pub registry_followup_types: Vec<Option<String>>, 
    pub primary_sponsor_types: Vec<Option<String>>,  
    pub primary_sponsor_names: Vec<Option<String>>, 
    pub primary_sponsor_countries: Vec<Option<String>>, 
    pub ethics_statuses: Vec<Option<String>>, 
    pub brief_summaries: Vec<Option<String>>, 
    pub trial_websites: Vec<Option<String>>, 
    pub publications: Vec<Option<String>>, 
    pub public_notess: Vec<Option<String>>, 
}

impl Trials{
    pub fn new(vsize: usize) -> Self {
        Trials { 
            trial_ids: Vec::with_capacity(vsize),
            actrn_ids: Vec::with_capacity(vsize),
            submit_dates: Vec::with_capacity(vsize),
            approval_dates: Vec::with_capacity(vsize),
            study_titles: Vec::with_capacity(vsize),
            scientific_titles: Vec::with_capacity(vsize),
            utns: Vec::with_capacity(vsize),
            trial_acronyms: Vec::with_capacity(vsize),
            linked_studies: Vec::with_capacity(vsize),
            study_types: Vec::with_capacity(vsize),
            patient_registrys: Vec::with_capacity(vsize),   
            registry_followups: Vec::with_capacity(vsize),
            registry_followup_types: Vec::with_capacity(vsize),
            primary_sponsor_types: Vec::with_capacity(vsize),
            primary_sponsor_names: Vec::with_capacity(vsize),
            primary_sponsor_countries: Vec::with_capacity(vsize),
            ethics_statuses: Vec::with_capacity(vsize),
            brief_summaries: Vec::with_capacity(vsize),
            trial_websites: Vec::with_capacity(vsize),
            publications: Vec::with_capacity(vsize),
            public_notess: Vec::with_capacity(vsize),                   
        }
    }

    pub fn add(&mut self, r: XLTrial) 
    {
        self.trial_ids.push(r.trial_id);
        self.actrn_ids.push(r.actrn_id);
        self.submit_dates.push(r.submit_date.clone());
        self.approval_dates.push(r.approval_date.clone());
        self.study_titles.push(r.study_title.clone());
        self.scientific_titles.push(r.scientific_title.clone());
        self.utns.push(r.utn.clone());
        self.trial_acronyms.push(r.trial_acronym.clone());
        self.linked_studies.push(r.linked_study.clone());
        self.study_types.push(r.study_type.clone());
        self.patient_registrys.push(r.patient_registry.clone());
        self.registry_followups.push(r.registry_followup.clone());
        self.registry_followup_types.push(r.registry_followup_type.clone());
        self.primary_sponsor_types.push(r.primary_sponsor_type.clone());
        self.primary_sponsor_names.push(r.primary_sponsor_name.clone());
        self.primary_sponsor_countries.push(r.primary_sponsor_country.clone());
        self.ethics_statuses.push(r.ethics_status.clone());
        self.brief_summaries.push(r.brief_summary.clone());
        self.trial_websites.push(r.trial_website.clone());
        self.publications.push(r.publication.clone());
        self.public_notess.push(r.public_notes.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.studies (trial_id, actrn_id, submit_date, approval_date, study_title, scientific_title, utn, trial_acronym, 
                        linked_study, study_type, patient_registry, registry_followup, registry_followup_type, 
                        primary_sponsor_type, primary_sponsor_name, primary_sponsor_country, 
                        ethics_status, brief_summary, trial_website, publication, public_notes) 
                        SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                        $7::text[], $8::text[], $9::text[], $10::text[], $11::text[], $12::text[], $13::text[], $14::text[],
                        $15::text[], $16::text[], $17::text[], $18::text[], $19::text[], $20::text[], $21::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.actrn_ids)
        .bind(&self.submit_dates)
        .bind(&self.approval_dates)
        .bind(&self.study_titles)
        .bind(&self.scientific_titles)
        .bind(&self.utns)
        .bind(&self.trial_acronyms)
        .bind(&self.linked_studies)
        .bind(&self.study_types)
        .bind(&self.patient_registrys)
        .bind(&self.registry_followups)
        .bind(&self.registry_followup_types)
        .bind(&self.primary_sponsor_types)
        .bind(&self.primary_sponsor_names)
        .bind(&self.primary_sponsor_countries)
        .bind(&self.ethics_statuses)
        .bind(&self.brief_summaries)
        .bind(&self.trial_websites)
        .bind(&self.publications)
        .bind(&self.public_notess)
        .execute(pool)

        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}



pub struct StudyLifeCycles {
    pub trial_ids: Vec<i32>, 
    pub actrn_ids: Vec<String>, 
    pub antic_start_dates: Vec<Option<String>>, 
    pub actual_start_dates: Vec<Option<String>>, 
    pub antic_end_dates: Vec<Option<String>>,  
    pub actual_end_dates: Vec<Option<String>>,  
    pub antic_last_visit_dates: Vec<Option<String>>, 
    pub actual_last_visit_dates: Vec<Option<String>>, 
    pub recruitment_statuses: Vec<Option<String>>, 
    pub data_analyses: Vec<Option<String>>, 
    pub withdrawn_reasons: Vec<Option<String>>, 
    pub withdrawn_reason_others: Vec<Option<String>>, 
    pub recruitment_countries: Vec<Option<String>>, 
    pub recruitment_states: Vec<Option<String>>, 
}

impl StudyLifeCycles{
    pub fn new(vsize: usize) -> Self {
        StudyLifeCycles { 
            trial_ids: Vec::with_capacity(vsize),
            actrn_ids: Vec::with_capacity(vsize),
            antic_start_dates: Vec::with_capacity(vsize),
            actual_start_dates: Vec::with_capacity(vsize),
            antic_end_dates: Vec::with_capacity(vsize),
            actual_end_dates: Vec::with_capacity(vsize),
            antic_last_visit_dates: Vec::with_capacity(vsize),
            actual_last_visit_dates: Vec::with_capacity(vsize),
            recruitment_statuses: Vec::with_capacity(vsize),
            data_analyses: Vec::with_capacity(vsize),
            withdrawn_reasons: Vec::with_capacity(vsize),    
            withdrawn_reason_others: Vec::with_capacity(vsize),
            recruitment_countries: Vec::with_capacity(vsize),
            recruitment_states: Vec::with_capacity(vsize),        
        }
    }

    pub fn add(&mut self, r: XLStudyLifeCycle) 
    {
        self.trial_ids.push(r.trial_id);
        self.actrn_ids.push(r.actrn_id);
        self.antic_start_dates.push(r.antic_start_date.clone());
        self.actual_start_dates.push(r.actual_start_date.clone());
        self.antic_end_dates.push(r.antic_end_date.clone());
        self.actual_end_dates.push(r.actual_end_date.clone());
        self.antic_last_visit_dates.push(r.antic_last_visit_date.clone());
        self.actual_last_visit_dates.push(r.actual_last_visit_date.clone());
        self.recruitment_statuses.push(r.recruitment_status.clone());
        self.data_analyses.push(r.data_analysis.clone());
        self.withdrawn_reasons.push(r.withdrawn_reason.clone());
        self.withdrawn_reason_others.push(r.withdrawn_reason_other.clone());
        self.recruitment_countries.push(r.recruitment_country.clone());
        self.recruitment_states.push(r.recruitmenbt_state.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.study_lifecycles (trial_id, actrn_id, antic_start_date, actual_start_date, antic_end_date, actual_end_date,
                        antic_last_visit_date, actual_last_visit_date, recruitment_status, data_analysis,
                        withdrawn_reason, withdrawn_reason_other, recruitment_country, recruitment_state ) 
                        SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                        $7::text[], $8::text[], $9::text[], $10::text[], $11::text[], $12::text[], $13::text[], $14::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.actrn_ids)
        .bind(&self.antic_start_dates)
        .bind(&self.actual_start_dates)
        .bind(&self.antic_end_dates)
        .bind(&self.actual_end_dates)
        .bind(&self.antic_last_visit_dates)
        .bind(&self.actual_last_visit_dates)
        .bind(&self.recruitment_statuses)
        .bind(&self.data_analyses)
        .bind(&self.withdrawn_reasons)
        .bind(&self.withdrawn_reason_others)
        .bind(&self.recruitment_countries)
        .bind(&self.recruitment_states)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct StudyFeaturess {
    pub trial_ids: Vec<i32>, 
    pub actrn_ids: Vec<String>, 
    pub interventionss: Vec<Option<String>>, 
    pub comparators: Vec<Option<String>>, 
    pub controls: Vec<Option<String>>, 
    pub purposes: Vec<Option<String>>, 
    pub allocations: Vec<Option<String>>, 
    pub concealments: Vec<Option<String>>, 
    pub sequencings: Vec<Option<String>>, 
    pub maskings: Vec<Option<String>>, 
    pub assignments:Vec<Option<String>>, 
    pub other_design_featuress: Vec<Option<String>>, 
    pub endpoints: Vec<Option<String>>, 
    pub phases: Vec<Option<String>>, 
    pub stat_methodss: Vec<Option<String>>, 
    pub masking_participantss: Vec<Option<String>>, 
    pub masking_clinicianss: Vec<Option<String>>, 
    pub masking_assessorss: Vec<Option<String>>, 
    pub masking_analystss: Vec<Option<String>>, 
    pub obs_purposes: Vec<Option<String>>, 
    pub obs_durations: Vec<Option<String>>, 
    pub obs_selections: Vec<Option<String>>, 
    pub obs_timings: Vec<Option<String>>, 
}

impl StudyFeaturess{
    pub fn new(vsize: usize) -> Self {
        StudyFeaturess { 
            trial_ids: Vec::with_capacity(vsize),
            actrn_ids: Vec::with_capacity(vsize),
            interventionss: Vec::with_capacity(vsize),
            comparators: Vec::with_capacity(vsize),
            controls: Vec::with_capacity(vsize),
            purposes: Vec::with_capacity(vsize),
            allocations: Vec::with_capacity(vsize),
            concealments: Vec::with_capacity(vsize),
            sequencings: Vec::with_capacity(vsize),
            maskings: Vec::with_capacity(vsize),
            assignments: Vec::with_capacity(vsize),    
            other_design_featuress: Vec::with_capacity(vsize),
            endpoints: Vec::with_capacity(vsize),
            phases: Vec::with_capacity(vsize),
            stat_methodss: Vec::with_capacity(vsize),
            masking_participantss: Vec::with_capacity(vsize),
            masking_clinicianss: Vec::with_capacity(vsize),
            masking_assessorss: Vec::with_capacity(vsize),
            masking_analystss: Vec::with_capacity(vsize),
            obs_purposes: Vec::with_capacity(vsize),     
            obs_durations: Vec::with_capacity(vsize),
            obs_selections: Vec::with_capacity(vsize),
            obs_timings: Vec::with_capacity(vsize),           
        }
    }

    pub fn add(&mut self, r: XLStudyFeatures) 
    {
        self.trial_ids.push(r.trial_id);
        self.actrn_ids.push(r.actrn_id);
        self.interventionss.push(r.interventions.clone());
        self.comparators.push(r.comparator.clone());
        self.controls.push(r.control.clone());
        self.purposes.push(r.purpose.clone());
        self.allocations.push(r.allocation.clone());
        self.concealments.push(r.concealment.clone());
        self.sequencings.push(r.sequencing.clone());
        self.maskings.push(r.masking.clone());
        self.assignments.push(r.assignment.clone());
        self.other_design_featuress.push(r.other_design_features.clone());
        self.endpoints.push(r.endpoint.clone());
        self.phases.push(r.phase.clone());
        self.stat_methodss.push(r.stat_methods.clone());
        self.masking_participantss.push(r.masking_participants.clone());
        self.masking_clinicianss.push(r.masking_clinicians.clone());
        self.masking_assessorss.push(r.masking_assessors.clone());
        self.masking_analystss.push(r.masking_analysts.clone());
        self.obs_purposes.push(r.obs_purpose.clone());
        self.obs_durations.push(r.obs_duration.clone());
        self.obs_selections.push(r.obs_selection.clone());
        self.obs_timings.push(r.obs_timing.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.study_features (trial_id, actrn_id, interventions, comparator, control, purpose, allocation, 
                        concealment, sequencing, masking, assignment, other_design_features, endpoint, phase, 
                        stat_methods, masking_participants, masking_clinicians, masking_assessors, masking_analysts, 
                        obs_purpose, obs_duration, obs_selection, obs_timing) 
                        SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                        $7::text[], $8::text[], $9::text[], $10::text[], $11::text[], $12::text[], $13::text[], $14::text[],
                        $15::text[], $16::text[], $17::text[], $18::text[], $19::text[], $20::text[], $21::text[], $22::text[], $23::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.actrn_ids)
        .bind(&self.interventionss)
        .bind(&self.comparators)
        .bind(&self.controls)
        .bind(&self.purposes)
        .bind(&self.allocations)
        .bind(&self.concealments)
        .bind(&self.sequencings)
        .bind(&self.maskings)
        .bind(&self.assignments)
        .bind(&self.other_design_featuress)
        .bind(&self.endpoints)
        .bind(&self.phases)
        .bind(&self.stat_methodss)
        .bind(&self.masking_participantss)
        .bind(&self.masking_clinicianss)
        .bind(&self.masking_assessorss)
        .bind(&self.masking_analystss)
        .bind(&self.obs_purposes)
        .bind(&self.obs_durations)
        .bind(&self.obs_selections)
        .bind(&self.obs_timings)

        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct Participantss {
    pub trial_ids: Vec<i32>, 
    pub actrn_ids: Vec<String>, 
    pub inclusion_criterias: Vec<Option<String>>, 
    pub min_ages: Vec<Option<String>>, 
    pub min_age_types: Vec<Option<String>>, 
    pub max_ages: Vec<Option<String>>, 
    pub max_age_types: Vec<Option<String>>, 
    pub genders: Vec<Option<String>>, 
    pub healthy_volunteerss: Vec<Option<String>>, 
    pub exclusion_criterias: Vec<Option<String>>, 
    pub target_sample_sizes: Vec<Option<String>>, 
    pub final_sample_sizes: Vec<Option<String>>, 
    pub current_sample_sizes: Vec<Option<String>>, 
}

impl Participantss{
    pub fn new(vsize: usize) -> Self {
        Participantss { 
            trial_ids: Vec::with_capacity(vsize),
            actrn_ids: Vec::with_capacity(vsize),
            inclusion_criterias: Vec::with_capacity(vsize),
            min_ages: Vec::with_capacity(vsize),
            min_age_types: Vec::with_capacity(vsize),
            max_ages: Vec::with_capacity(vsize),
            max_age_types: Vec::with_capacity(vsize),
            genders: Vec::with_capacity(vsize),
            healthy_volunteerss: Vec::with_capacity(vsize),
            exclusion_criterias: Vec::with_capacity(vsize),
            target_sample_sizes: Vec::with_capacity(vsize),   
            final_sample_sizes: Vec::with_capacity(vsize),
            current_sample_sizes: Vec::with_capacity(vsize),           
        }
    }

    pub fn add(&mut self, r: XLParticipants) 
    {
        self.trial_ids.push(r.trial_id);
        self.actrn_ids.push(r.actrn_id);
        self.inclusion_criterias.push(r.inclusion_criteria.clone());
        self.min_ages.push(r.min_age.clone());
        self.min_age_types.push(r.min_age_type.clone());
        self.max_ages.push(r.max_age.clone());
        self.max_age_types.push(r.max_age_type.clone());
        self.genders.push(r.gender.clone());
        self.healthy_volunteerss.push(r.healthy_volunteers.clone());
        self.exclusion_criterias.push(r.final_sample_size.clone());
        self.target_sample_sizes.push(r.target_sample_size.clone());
        self.final_sample_sizes.push(r.final_sample_size.clone());
        self.current_sample_sizes.push(r.current_sample_size.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.study_participants (trial_id, actrn_id, inclusion_criteria, min_age, min_age_type, max_age, max_age_type, gender,
                        healthy_volunteers, exclusion_criteria, target_sample_size, final_sample_size, current_sample_size)
                        SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                        $7::text[], $8::text[], $9::text[], $10::text[], $11::text[], $12::text[], $13::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.actrn_ids)
        .bind(&self.inclusion_criterias)
        .bind(&self.min_ages)
        .bind(&self.min_age_types)
        .bind(&self.max_ages)
        .bind(&self.max_age_types)
        .bind(&self.genders)
        .bind(&self.healthy_volunteerss)
        .bind(&self.exclusion_criterias)
        .bind(&self.target_sample_sizes)
        .bind(&self.final_sample_sizes)
        .bind(&self.current_sample_sizes)       
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}




pub struct SingleDataFields {
    pub trial_ids: Vec<i32>, 
    pub data_fields: Vec<Option<String>>, 
}

impl SingleDataFields{
    pub fn new(vsize: usize) -> Self {
        SingleDataFields { 
            trial_ids: Vec::with_capacity(vsize),
            data_fields: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLSingleDataField) 
    {
        self.trial_ids.push(r.trial_id);
        self.data_fields.push(r.data_field.clone());
    }

    pub async fn store_data(&self, table_name: &str, data_field: &str, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = format!(r#"INSERT INTO xl.{} (trial_id, {}) 
            SELECT * FROM UNNEST($1::int[], $2::text[])"#, table_name, data_field);

        sqlx::query(&sql)
        .bind(&self.trial_ids)
        .bind(&self.data_fields)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql))
    }
}


pub struct DoubleDataFields {
    pub trial_ids: Vec<i32>, 
    pub data_field1s: Vec<Option<String>>, 
    pub data_field2s: Vec<Option<String>>, 
}

impl DoubleDataFields{
    pub fn new(vsize: usize) -> Self {
        DoubleDataFields { 
            trial_ids: Vec::with_capacity(vsize),
            data_field1s: Vec::with_capacity(vsize),
            data_field2s: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLDoubleDataField) 
    {
        self.trial_ids.push(r.trial_id);
        self.data_field1s.push(r.data_field1);
        self.data_field2s.push(r.data_field2);
    }

    pub async fn store_data(&self, table_name: &str, data_field1: &str, data_field2: &str, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = format!(r#"INSERT INTO xl.{} (trial_id, {}, {}) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[])"#, table_name, data_field1, data_field2);

        sqlx::query(&sql)
        .bind(&self.trial_ids)
        .bind(&self.data_field1s)
        .bind(&self.data_field2s)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql))
    }
}


pub struct Outcomes {
    pub trial_ids: Vec<i32>,
    pub outcomes: Vec<Option<String>>, 
    pub outcome_assessments: Vec<Option<String>>, 
    pub timepoints: Vec<Option<String>>, 
}

impl Outcomes{
    pub fn new(vsize: usize) -> Self {
        Outcomes { 
            trial_ids: Vec::with_capacity(vsize),
            outcomes: Vec::with_capacity(vsize),
            outcome_assessments: Vec::with_capacity(vsize),
            timepoints: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLOutcome) 
    {
        self.trial_ids.push(r.trial_id);
        self.outcomes.push(r.outcome.clone());
        self.outcome_assessments.push(r.outcome_assessment.clone());
        self.timepoints.push(r.timepoint.clone());
    }

    pub async fn store_data(&self, table_name: &str, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = format!(r#"INSERT INTO xl.{} (trial_id, outcome, outcome_assessment, timepoint) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[])"#, table_name);

        sqlx::query(&sql)
        .bind(&self.trial_ids)
        .bind(&self.outcomes)
        .bind(&self.outcome_assessments)
        .bind(&self.timepoints)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql))
    }
}


pub struct TypeNameCountries {
    pub trial_ids: Vec<i32>,
    pub entity_types: Vec<Option<String>>, 
    pub names: Vec<Option<String>>, 
    pub countries: Vec<Option<String>>, 
}

impl TypeNameCountries{
    pub fn new(vsize: usize) -> Self {
        TypeNameCountries { 
            trial_ids: Vec::with_capacity(vsize),
            entity_types: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            countries: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLTypeNameCountry) 
    {
        self.trial_ids.push(r.trial_id);
        self.entity_types.push(r.entity_type.clone());
        self.names.push(r.name.clone());
        self.countries.push(r.country.clone());

    }

    pub async fn store_data(&self, table_name: &str, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = format!(r#"INSERT INTO xl.{} (trial_id, type, name, country) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[])"#, table_name);

        sqlx::query(&sql)
        .bind(&self.trial_ids)
        .bind(&self.entity_types)
        .bind(&self.names)
        .bind(&self.countries)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql))
    }
}


pub struct EthicsCommittees {
    pub trial_ids: Vec<i32>,
    pub names: Vec<Option<String>>, 
    pub addresses: Vec<Option<String>>, 
    pub countries: Vec<Option<String>>, 
    pub submit_dates: Vec<Option<String>>, 
    pub approval_dates: Vec<Option<String>>, 
    pub hrec_approval_ids: Vec<Option<String>>, 
}

impl EthicsCommittees{
    pub fn new(vsize: usize) -> Self {
        EthicsCommittees { 
            trial_ids: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            addresses: Vec::with_capacity(vsize),
            countries: Vec::with_capacity(vsize),
            submit_dates: Vec::with_capacity(vsize),
            approval_dates: Vec::with_capacity(vsize),
            hrec_approval_ids: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLEthicsCommittee) 
    {
        self.trial_ids.push(r.trial_id);
        self.names.push(r.name.clone());
        self.addresses.push(r.name.clone());
        self.countries.push(r.country.clone());
        self.submit_dates.push(r.submit_date.clone());
        self.approval_dates.push(r.approval_date.clone());
        self.hrec_approval_ids.push(r.hrec_approval_id.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.ethics_committees (trial_id, name, address, country,
                                     submit_date, approval_date, hrec_approval_id) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.names)
        .bind(&self.addresses)
        .bind(&self.countries)
        .bind(&self.submit_dates)
        .bind(&self.approval_dates)
        .bind(&self.hrec_approval_ids)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct Contacts {
    pub trial_ids: Vec<i32>,
    pub contact_types: Vec<Option<String>>, 
    pub titles: Vec<Option<String>>,
    pub names: Vec<Option<String>>, 
    pub addresses: Vec<Option<String>>, 
    pub countries: Vec<Option<String>>, 
    pub phones: Vec<Option<String>>, 
    pub faxes: Vec<Option<String>>, 
    pub emails: Vec<Option<String>>, 
}

impl Contacts{
    pub fn new(vsize: usize) -> Self {
        Contacts { 
            trial_ids: Vec::with_capacity(vsize),
            contact_types: Vec::with_capacity(vsize),
            titles: Vec::with_capacity(vsize),
            names: Vec::with_capacity(vsize),
            addresses: Vec::with_capacity(vsize),
            countries: Vec::with_capacity(vsize),
            phones: Vec::with_capacity(vsize),
            faxes: Vec::with_capacity(vsize),
            emails: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLContact) 
    {
        self.trial_ids.push(r.trial_id);
        self.contact_types.push(r.contact_type.clone());
        self.titles.push(r.title.clone());
        self.names.push(r.name.clone());
        self.addresses.push(r.address.clone());
        self.countries.push(r.country.clone());
        self.phones.push(r.phone.clone());
        self.faxes.push(r.fax.clone());
        self.emails.push(r.email.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.contacts (trial_id, type, title, name, address, country, phone, fax, email) 
                                 SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[],
                                 $5::text[], $6::text[], $7::text[], $8::text[], $9::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.contact_types)
        .bind(&self.titles)
        .bind(&self.names)
        .bind(&self.addresses)
        .bind(&self.countries)
        .bind(&self.phones)
        .bind(&self.faxes) 
        .bind(&self.emails)    
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct DSSs {
    pub trial_ids: Vec<i32>,
    pub ipd_availabilities: Vec<Option<String>>, 
    pub available_to_who: Vec<Option<String>>, 
    pub availability_conds: Vec<Option<String>>, 
    pub data_to_be_shareds: Vec<Option<String>>, 
    pub for_what_analyses: Vec<Option<String>>, 
    pub timeframe_froms: Vec<Option<String>>, 
    pub timeframe_tos: Vec<Option<String>>, 
    pub mechanisms: Vec<Option<String>>, 
    pub extras: Vec<Option<String>>, 
}

impl DSSs{
    pub fn new(vsize: usize) -> Self {
        DSSs { 
            trial_ids: Vec::with_capacity(vsize),
            ipd_availabilities: Vec::with_capacity(vsize),
            available_to_who: Vec::with_capacity(vsize),
            availability_conds: Vec::with_capacity(vsize),
            data_to_be_shareds: Vec::with_capacity(vsize),
            for_what_analyses: Vec::with_capacity(vsize),
            timeframe_froms: Vec::with_capacity(vsize),
            timeframe_tos: Vec::with_capacity(vsize),
            mechanisms: Vec::with_capacity(vsize),
            extras: Vec::with_capacity(vsize),           
        }
    }

    pub fn add(&mut self, r: XLDSS) 
    {
        self.trial_ids.push(r.trial_id);
        self.ipd_availabilities.push(r.ipd_availability.clone());
        self.available_to_who.push(r.available_to_whom.clone());
        self.availability_conds.push(r.availability_conditions.clone());
        self.data_to_be_shareds.push(r.data_to_be_shared.clone());
        self.for_what_analyses.push(r.for_what_analyses_types.clone());
        self.timeframe_froms.push(r.timeframe_from.clone());
        self.timeframe_tos.push(r.timeframe_to.clone());
        self.mechanisms.push(r.mechanism.clone());
        self.extras.push(r.extra_considerations.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.data_sharing_statements (trial_id, ipd_availability, available_to_whom, availability_conditions, data_to_be_shared, 
                                    for_what_analyses_types, timeframe_from, timeframe_to, mechanism, extra_considerations) 
                                    SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[],
                                    $7::text[], $8::text[], $9::text[], $10::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.ipd_availabilities)
        .bind(&self.available_to_who)
        .bind(&self.availability_conds)
        .bind(&self.data_to_be_shareds)
        .bind(&self.for_what_analyses)
        .bind(&self.timeframe_froms)
        .bind(&self.timeframe_tos)
        .bind(&self.mechanisms)
        .bind(&self.extras)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct SuppDocs {
    pub trial_ids: Vec<i32>,
    pub doc_types: Vec<Option<String>>, 
    pub citations: Vec<Option<String>>, 
    pub links: Vec<Option<String>>, 
    pub emails: Vec<Option<String>>, 
    pub detailss: Vec<Option<String>>, 
    pub attachments: Vec<Option<String>>, 
}

impl SuppDocs{
    pub fn new(vsize: usize) -> Self {
        SuppDocs { 
            trial_ids: Vec::with_capacity(vsize),
            doc_types: Vec::with_capacity(vsize),
            citations: Vec::with_capacity(vsize),
            links: Vec::with_capacity(vsize),
            emails: Vec::with_capacity(vsize),
            detailss: Vec::with_capacity(vsize),
            attachments: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLSuppDoc) 
    {
        self.trial_ids.push(r.trial_id);
        self.doc_types.push(r.doc_type.clone());
        self.citations.push(r.citation.clone());
        self.links.push(r.link.clone());
        self.emails.push(r.email.clone());
        self.detailss.push(r.details.clone());
        self.attachments.push(r.attachment.clone());

    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.supporting_documents (trial_id, type, citation, link, email, details, attachment) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.doc_types)
        .bind(&self.citations)
        .bind(&self.links)
        .bind(&self.emails)
        .bind(&self.detailss)
        .bind(&self.attachments)        
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct StudyResults {
    pub trial_ids: Vec<i32>,
    pub results_types: Vec<Option<String>>, 
    pub is_peer_revieweds: Vec<Option<String>>, 
    pub dois: Vec<Option<String>>, 
    pub citations_or_detailss: Vec<Option<String>>, 
    pub attachments: Vec<Option<String>>, 
}

impl StudyResults{
    pub fn new(vsize: usize) -> Self {
        StudyResults { 
            trial_ids: Vec::with_capacity(vsize),
            results_types: Vec::with_capacity(vsize),
            is_peer_revieweds: Vec::with_capacity(vsize),
            dois: Vec::with_capacity(vsize),
            citations_or_detailss: Vec::with_capacity(vsize),
            attachments: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLStudyResult) 
    {
        self.trial_ids.push(r.trial_id);
        self.results_types.push(r.results_type.clone());
        self.is_peer_revieweds.push(r.is_peer_reviewed.clone());
        self.dois.push(r.doi.clone());
        self.citations_or_detailss.push(r.citations_or_details.clone());
        self.attachments.push(r.attachment.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.study_results (trial_id, type, is_peer_reviewed, doi, citations_or_details, attachment) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[], $6::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.results_types)
        .bind(&self.is_peer_revieweds)
        .bind(&self.dois)
        .bind(&self.citations_or_detailss)
        .bind(&self.attachments)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}


pub struct ExternalPublications {
    pub trial_ids: Vec<i32>,
    pub sources: Vec<Option<String>>, 
    pub dois: Vec<Option<String>>, 
    pub titles: Vec<Option<String>>, 
    pub year_of_publications: Vec<Option<String>>, 
}

impl ExternalPublications{
    pub fn new(vsize: usize) -> Self {
        ExternalPublications { 
            trial_ids: Vec::with_capacity(vsize),
            sources: Vec::with_capacity(vsize),
            dois: Vec::with_capacity(vsize),
            titles: Vec::with_capacity(vsize),
            year_of_publications: Vec::with_capacity(vsize),
        }
    }

    pub fn add(&mut self, r: XLExternalPublication) 
    {
        self.trial_ids.push(r.trial_id);
        self.sources.push(r.source.clone());
        self.dois.push(r.doi.clone());
        self.titles.push(r.title.clone());
        self.year_of_publications.push(r.year_of_publication.clone());
    }

    pub async fn store_data(&self, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = r#"INSERT INTO xl.external_publications (trial_id, source, doi, title, year_of_publication) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[], $5::text[])"#;

        sqlx::query(sql)
        .bind(&self.trial_ids)
        .bind(&self.sources)
        .bind(&self.dois)
        .bind(&self.titles)
        .bind(&self.year_of_publications)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))
    }
}
