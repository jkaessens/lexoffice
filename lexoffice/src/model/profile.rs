#![doc = "The profile endpoint provides read access to basic profile information such as company name, user id, name and email of the connected lexoffice account."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DistanceSalesPrinciple {
    #[serde(rename = "DESTINATION")]
    Destination,
    #[serde(rename = "ORIGIN")]
    Origin,
}
impl std::str::FromStr for DistanceSalesPrinciple {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxType {
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "net")]
    Net,
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
    #[doc = "The distance sales principle configured by the user, or undefined if not yet set. Possible values are **ORIGIN** and **DESTINATION**. See the in-app documentation (Section \"Umsatzsteuer bei Privatpersonen im EU-Ausland\" in [https://app.lexoffice.de/settings/#/general](https://app.lexoffice.de/settings/#/general)) for information about the implications of this setting."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub distance_sales_principle: Option<DistanceSalesPrinciple>,
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
