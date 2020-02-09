use crate::model::server_resource::ServerResource;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Page<T> {
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
