use crate::{
    get,
    response::{CollectionResponse, ResourceResponse},
    wanikani_client::WanikaniClient,
    wanikani_client::QP,
};
use chrono::{DateTime, Utc};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use wanikani_rs_model::subject::Subject;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectFilter {
    Ids(Vec<i64>),
    Types(Vec<String>),
    Slugs(Vec<String>),
    Levels(Vec<i64>),
    Hidden(bool),
    UpdatedAfter(DateTime<Utc>),
}

impl WanikaniClient {
    get!(
        get_subjects_filtered,
        "subjects",
        SubjectFilter,
        CollectionResponse<ResourceResponse<Subject>>
    );
    get!(
        get_subjects,
        "subjects",
        CollectionResponse<ResourceResponse<Subject>>
    );
    get!(
        get_subject,
        "subjects/{id}",
        ResourceResponse<Subject>,
        id: i64
    );
}
