
SET client_min_messages TO WARNING; 
create schema if not exists xl;


drop table if exists xl.studies;
create table xl.studies (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , submit_date              varchar
  , approval_date            varchar
  , study_title              varchar
  , scientific_title         varchar
  , utn                      varchar
  , trial_acronym            varchar
  , linked_study             varchar
  , study_type               varchar
  , patient_registry         varchar
  , registry_followup        varchar
  , registry_followup_type   varchar
  , primary_sponsor_type     varchar
  , primary_sponsor_name     varchar
  , primary_sponsor_country  varchar
  , ethics_status            varchar
  , brief_summary            varchar
  , trial_website            varchar
  , publication              varchar
  , public_notes             varchar
}
create index tid on xl.trials(trial_id);
create index xl_sid on xl.trials(actrn_id);


drop table if exists xl.study_lifecycles;
create table xl.study_lifecycles (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , antic_start_date         varchar
  , actual_start_date        varchar
  , antic_end_date           varchar
  , actual_end_date          varchar
  , antic_last_visit_date    varchar
  , actual_last_visit_date   varchar
  , recruitment_status       varchar
  , data_analysis            varchar
  , withdrawn_reason         varchar
  , withdrawn_reason_other   varchar
  , recruitment_country      varchar
  , recruitment_state        varchar
}
create index lc_tid on xl.study_lifecycles(trial_id);
create index lc_sid on xl.study_lifecycles(actrn_id);


drop table if exists xl.study_features;
create table xl.study_features (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , interventions            varchar
  , comparator               varchar
  , control                  varchar
  , purpose                  varchar
  , allocation               varchar
  , concealment              varchar
  , sequencing               varchar
  , masking                  varchar
  , assignment               varchar
  , other_design_features    varchar
  , endpoint                 varchar
  , phase                    varchar
  , stat_methods             varchar
  , masking_participants     varchar
  , masking_clinicians       varchar
  , masking_assessors        varchar
  , masking_analysts         varchar
  , obs_purpose              varchar
  , obs_duration             varchar
  , obs_selection            varchar
  , obs_timing               varchar
}
create index sf_tid on xl.study_features(trial_id);
create index sf_sid on xl.study_features(actrn_id);


drop table if exists xl.study_participants;
create table xl.study_participants (
    trial_id                 int        primary key
  , actrn_id                 varchar
  , inclusion_criteria       varchar
  , min_age                  varchar
  , min_age_type             varchar
  , max_age                  varchar
  , max_age_type             varchar
  , gender                   varchar
  , healthy_volunteers       varchar
  , exclusion_criteria       varchar
  , target_sample_size       varchar
  , final_sample_size        varchar
  , current_sample_size      varchar
}
create index sp_tid on xl.study_participants(trial_id);
create index sp_sid on xl.study_participants(actrn_id);


drop table if exists xl.trials;
create table xl.trials (
    
  , purpose                  varchar
  , allocation               varchar
  , concealment              varchar
  , sequencing               varchar
  , masking                  varchar
  , assignment               varchar
  , other_design_features    varchar
  , endpoint                 varchar
  , phase                    varchar
  , stat_methods             varchar
  , masking_participants     varchar
  , masking_clinicians       varchar
  , masking_assessors        varchar
  , masking_analysts         varchar
  , patient_registry         varchar
  , registry_followup        varchar
  , registry_followup_type   varchar
  , obs_purpose              varchar
  , obs_duration             varchar
  , obs_selection            varchar
  , obs_timing               varchar
);
create index tid on xl.trials(trial_id);
create index xl_sid on xl.trials(actrn_id);


drop table if exists xl.secondary_ids;
create table xl.secondary_ids (
    trial_id                 int
  , sec_id                   varchar
);
create index sec_ids_id on xl.secondary_ids(trial_id);


drop table if exists xl.health_conditions;
create table xl.health_conditions (
    trial_id                 int
  , health_condition         varchar
);
create index health_conditions_id on xl.health_conditions(trial_id);


drop table if exists xl.condition_codes;
create table xl.condition_codes (
    trial_id                 int
  , condition_category       varchar
  , condition_code           varchar
);
create index condition_codes_id on xl.condition_codes(trial_id);


drop table if exists xl.intervention_codes;
create table xl.intervention_codes (
    trial_id                 int
  , intervention_code        varchar
);
create index intervention_codes_id on xl.intervention_codes(trial_id);


drop table if exists xl.primary_outcomes;
create table xl.primary_outcomes (
    trial_id                 int
  , outcome                  varchar
  , outcome_assessment       varchar
  , timepoint                varchar
);
create index primary_outcomes_id on xl.primary_outcomes(trial_id);


drop table if exists xl.secondary_outcomes;
create table xl.secondary_outcomes (
    trial_id                 int
  , outcome                  varchar
  , outcome_assessment       varchar
  , timepoint                varchar
);
create index secondary_outcomes_id on xl.secondary_outcomes(trial_id);


drop table if exists xl.hospitals;
create table xl.hospitals (
    trial_id                 int
  , hospital                 varchar
);
create index hospitals_id on xl.hospitals(trial_id);


drop table if exists xl.other_countries;
create table xl.other_countries (
    trial_id                 int
  , country                  varchar
  , state                    varchar
);
create index other_countries_id on xl.other_countries(trial_id);


drop table if exists xl.funding_sources;
create table xl.funding_sources (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index funding_sources_id on xl.funding_sources(trial_id);


drop table if exists xl.secondary_sponsors;
create table xl.secondary_sponsors (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index secondary_sponsors_id on xl.secondary_sponsors(trial_id);


drop table if exists xl.other_collaborators;
create table xl.other_collaborators (
    trial_id                 int
  , type                     varchar
  , name                     varchar
  , country                  varchar
);
create index other_collaborators_id on xl.other_collaborators(trial_id);


drop table if exists xl.ethics_committees;
create table xl.ethics_committees (
    trial_id                 int
  , name                     varchar
  , address                  varchar
  , country                  varchar
  , submit_date              varchar
  , approval_date            varchar
  , hrec_approval_id         varchar
);
create index ethics_committees_id on xl.ethics_committees(trial_id);


drop table if exists xl.contacts;
create table xl.contacts (
    trial_id                 int
  , type                     varchar
  , title                    varchar
  , name                     varchar
  , address                  varchar
  , country                  varchar
  , phone                    varchar
  , fax                      varchar
  , email                    varchar
);
create index contacts_id on xl.contacts(trial_id);


drop table if exists xl.data_sharing_statements;
create table xl.data_sharing_statements (
    trial_id                 int
  , ipd_availability         varchar
  , available_to_whom        varchar
  , availability_conditions  varchar
  , data_to_be_shared        varchar
  , for_what_analyses_types  varchar
  , timeframe_from           varchar
  , timeframe_to             varchar
  , mechanism                varchar
  , extra_considerations     varchar
);
create index data_sharing_statements_id on xl.data_sharing_statements(trial_id);


drop table if exists xl.supporting_documents;
create table xl.supporting_documents (
    trial_id                 int
  , type                     varchar
  , citation                 varchar
  , link                     varchar
  , email                    varchar
  , details                  varchar
  , attachment               varchar
);
create index supporting_documents_id on xl.supporting_documents(trial_id);


drop table if exists xl.study_results;
create table xl.study_results (
    trial_id                 int
  , type                     varchar
  , is_peer_reviewed         varchar
  , doi      	               varchar
  , citations_or_details     varchar
  , attachment               varchar
);
create index study_results_id on xl.study_results(trial_id);


drop table if exists xl.external_publications;
create table xl.external_publications (
    trial_id                 int
  , source                   varchar
  , doi                      varchar
  , title                    varchar
  , year_of_publication      varchar
);
create index external_publications_id on xl.external_publications(trial_id);


SET client_min_messages TO NOTICE; 


-- Not required
--drop table if exists xl.postcodes;
--create table xl.postcodes (
--    trial_id				    int
-- , postcode        	    	varchar
--);
--create index postcodes_id on xl.postcodes(trial_id);