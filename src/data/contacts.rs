use crate::resource::{PaginatedResource, Resource, ItemsResource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum SalutationEnum {
    #[serde(rename = "Herr")]
    Male,
    #[serde(rename = "Frau")]
    Female,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    version: Option<i64>,
    roles: RolesDetails,
    company: CompanyDetails,
    person: Option<PersonDetails>,
    #[serde(default)]
    addresses: AddressesDetails,
    #[serde(default)]
    email_addresses: EMailAddressesDetails,
    #[serde(default)]
    phone_numbers: PhoneNumbersDetails,
    note: Option<String>,
    archived: Option<bool>,
}

impl Resource for Contact {
    const BASE_PATH: &'static str = "contacts";
}
impl ItemsResource for Contact {
}
impl PaginatedResource for Contact {
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesDetails {
    customer: Option<CustomerDetails>,
    vendor: Option<VendorDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDetails {
    number: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorDetails {
    number: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyDetails {
    #[serde(default)]
    allow_tax_free_invoices: bool,
    name: String,
    tax_number: Option<String>,
    vat_registration_id: Option<String>,
    #[serde(default)]
    contact_persons: Vec<CompanyContactPersonDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyContactPersonDetails {
    salutation: SalutationEnum,
    first_name: String,
    last_name: String,
    email_address: String,
    phone_number: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonDetails {
    salutation: SalutationEnum,
    first_name: String,
    last_name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AddressesDetails {
    billing: Vec<AddressDetails>,
    shipping: Vec<AddressDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressDetails {
    supplement: Option<String>,
    street: Option<String>,
    zip: Option<String>,
    city: Option<String>,
    country_code: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EMailAddressesDetails {
    business: Vec<String>,
    office: Vec<String>,
    private: Vec<String>,
    other: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PhoneNumbersDetails {
    business: Vec<String>,
    office: Vec<String>,
    mobile: Vec<String>,
    private: Vec<String>,
    fax: Vec<String>,
    other: Vec<String>,
}
