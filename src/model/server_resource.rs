use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResource<T> {
    id: Uuid,
    organization_id: Uuid,
    version: u64,
    archived: Option<bool>,

    #[serde(flatten)]
    modification_info: Option<ModificationInfo>,
    #[serde(flatten)]
    value: T,
}

impl<T> Deref for ServerResource<T> {
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
