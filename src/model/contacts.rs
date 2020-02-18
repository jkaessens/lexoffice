use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SalutationEnum {
    #[serde(rename = "Herr")]
    Male,
    #[serde(rename = "Frau")]
    Female,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Contact {
    pub organization_id: Uuid,
    pub roles: RolesDetails,
    pub company: CompanyDetails,
    pub person: Option<PersonDetails>,
    #[serde(default)]
    pub addresses: AddressesDetails,
    #[serde(default)]
    pub email_addresses: EMailAddressesDetails,
    #[serde(default)]
    pub phone_numbers: PhoneNumbersDetails,
    pub note: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RolesDetails {
    pub customer: Option<CustomerDetails>,
    pub vendor: Option<VendorDetails>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CustomerDetails {
    pub number: i64,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VendorDetails {
    pub number: i64,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompanyDetails {
    #[serde(default)]
    pub allow_tax_free_invoices: bool,
    pub name: String,
    pub tax_number: Option<String>,
    pub vat_registration_id: Option<String>,
    #[serde(default)]
    pub contact_persons: Vec<CompanyContactPersonDetails>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompanyContactPersonDetails {
    pub salutation: SalutationEnum,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub phone_number: Option<String>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PersonDetails {
    pub salutation: SalutationEnum,
    pub first_name: String,
    pub last_name: String,
}
#[derive(
    Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize, Default,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase", default)]
pub struct AddressesDetails {
    pub billing: Vec<AddressDetails>,
    pub shipping: Vec<AddressDetails>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddressDetails {
    pub supplement: Option<String>,
    pub street: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub country_code: String,
}
#[derive(
    Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize, Default,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase", default)]
pub struct EMailAddressesDetails {
    pub business: Vec<String>,
    pub office: Vec<String>,
    pub private: Vec<String>,
    pub other: Vec<String>,
}
#[derive(
    Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize, Default,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase", default)]
pub struct PhoneNumbersDetails {
    pub business: Vec<String>,
    pub office: Vec<String>,
    pub mobile: Vec<String>,
    pub private: Vec<String>,
    pub fax: Vec<String>,
    pub other: Vec<String>,
}
