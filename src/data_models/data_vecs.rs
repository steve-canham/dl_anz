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


    pub fn shrink_to_fit(&mut self) -> () {
        self.trial_ids.shrink_to_fit();
        self.data_fields.shrink_to_fit();
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


    pub fn shrink_to_fit(&mut self) -> () {
        self.trial_ids.shrink_to_fit();
        self.data_field1s.shrink_to_fit();
        self.data_field2s.shrink_to_fit();
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

    pub fn shrink_to_fit(&mut self) -> () {
        self.trial_ids.shrink_to_fit();
        self.outcomes.shrink_to_fit();
        self.outcome_assessments.shrink_to_fit();
        self.timepoints.shrink_to_fit();
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

    pub fn shrink_to_fit(&mut self) -> () {
        self.trial_ids.shrink_to_fit();
        self.entity_types.shrink_to_fit();
        self.names.shrink_to_fit();
        self.countries.shrink_to_fit();
    }

    pub async fn store_data(&self, table_name: &str, entity_type: &str, pool : &Pool<Postgres>) -> Result<PgQueryResult, AppError> {

        let sql = format!(r#"INSERT INTO xl.{} (trial_id, {}, name, country) 
            SELECT * FROM UNNEST($1::int[], $2::text[], $3::text[], $4::text[])"#, table_name, entity_type);

        sqlx::query(&sql)
        .bind(&self.trial_ids)
        .bind(&self.entity_types)
        .bind(&self.names)
        .bind(&self.countries)
        .execute(pool)
        .await.map_err(|e| AppError::SqlxError(e, sql))
    }
}
