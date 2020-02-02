use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSubscription {
    subscription_id: Uuid,
    organization_id: Uuid,
    created_date: DateTime<Utc>,
    event_type: String,
    callback_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookCallback {
    organization_id: Uuid,
    event_type: String,
    resource_id: Uuid,
    event_date: DateTime<Utc>,
}
