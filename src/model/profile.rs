use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum TaxTypeEnum {
    Net,
    Gross,
    Vatfree,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Profile {
    pub organization_id: Uuid,
    pub company_name: String,
    pub created: CreatedDetails,
    pub connection_id: Uuid,
    pub tax_type: TaxTypeEnum,
    pub small_business: bool,
}

#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreatedDetails {
    pub user_name: String,
    pub user_email: String,
    pub date: String,
}
