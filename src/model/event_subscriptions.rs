use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventSubscription {
    pub subscription_id: Uuid,
    pub organization_id: Uuid,
    pub created_date: DateTime<Utc>,
    pub event_type: String,
    pub callback_url: String,
}

#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebhookCallback {
    pub organization_id: Uuid,
    pub event_type: String,
    pub resource_id: Uuid,
    pub event_date: DateTime<Utc>,
}
