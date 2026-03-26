use crate::data_models::xl_data_models::*;
use crate::AppError;
use sqlx::{Pool, Postgres, postgres::PgQueryResult};
//use chrono::{NaiveDate, NaiveDateTime};


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
