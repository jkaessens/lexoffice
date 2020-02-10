use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ServerResource<T> {
    pub id: Option<Uuid>,
    pub version: Option<u64>,

    #[serde(flatten)]
    pub inner: T,
}

impl<T> Deref for ServerResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
