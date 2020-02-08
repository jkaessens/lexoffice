use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum TaxTypeEnum {
    Net,
    Gross,
    Vatfree,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    company_name: String,
    created: CreatedDetails,
    connection_id: Uuid,
    tax_type: TaxTypeEnum,
    small_business: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedDetails {
    user_name: String,
    user_email: String,
    date: String,
}
