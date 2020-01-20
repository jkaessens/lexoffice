use crate::resource::Resource;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResource<T>
where
    T: Resource,
{
    id: Uuid,
    organization_id: Uuid,
    version: u64,
    archived: Option<bool>,
    #[serde(flatten)]
    modification_info: Option<ModificationInfo>,
    #[serde(flatten)]
    value: T,
}

impl<T> Deref for ServerResource<T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModificationInfo {
    created_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T>
where T: Resource {
    pub content: Vec<ServerResource<T>>,
    #[serde(default = "default_true")]
    pub last: bool,
    #[serde(default)]
    pub total_pages: usize,
    #[serde(default)]
    pub total_elements: usize,
    #[serde(default)]
    pub sort: serde_json::Value,
    #[serde(default)]
    pub size: usize,
    #[serde(default)]
    pub number: usize,
    #[serde(default = "default_true")]
    pub first: bool,
    #[serde(default)]
    pub number_of_elements: usize,
}

fn default_true() -> bool {
    true
}
