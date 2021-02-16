#![doc = "The profile endpoint provides read access to basic profile information such as company name, user id, name and email of the connected lexoffice account."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxType {
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "vatfree")]
    Vatfree,
}
impl std::str::FromStr for TaxType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Profile {
    #[doc = "Unique id of your company."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_id: Option<uuid::Uuid>,
    #[doc = "Name of your company registered at lexoffice."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub company_name: Option<String>,
    #[doc = "Information about the established connection to lexoffice. For specification of object `created` please see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub created: Option<Created>,
    #[doc = "The id of the current API connection."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub connection_id: Option<uuid::Uuid>,
    #[doc = "Configured tax type. Possible values are **net**, **gross**, and **vatfree**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type: Option<TaxType>,
    #[doc = "Reflects whether the organization is marked as a \"small business\" (*Kleinunternehmer* according to §19 UStG.)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub small_business: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Created {
    #[doc = "Unique id of the user who established the connection to lexoffice."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "The user who established the connection to lexoffice."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_name: Option<String>,
    #[doc = "The user's email who established the connection to lexoffice."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub user_email: Option<String>,
    #[doc = "The date when the connection was established in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub date: Option<String>,
}
