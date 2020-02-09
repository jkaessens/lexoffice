use celes::Country;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
enum CurrencyEnum {
    EUR,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
enum TaxTypeEnum {
    Net,
    Gross,
    Vatfree,
    IntraCommunitySupply,
    ConstructionService13b,
    ExternalService13b,
    ThirdPartyCountryService,
    ThirdPartyCountryDelivery,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
enum TypeEnum {
    Service,
    Material,
    Custom,
    Text,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
enum VoucherStatusEnum {
    Draft,
    Open,
    Accepted,
    Rejected,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Quotation {
    id: Option<Uuid>,
    organization_id: Uuid,
    created_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    expiration_date: DateTime<Utc>,
    version: i64,
    language: String,
    archived: bool,
    voucher_status: VoucherStatusEnum,
    voucher_number: String,
    voucher_date: DateTime<Utc>,
    address: AddressDetails,
    line_items: Vec<LineItemsDetails>,
    total_price: TotalPriceDetails,
    tax_amounts: Vec<TaxAmountsDetails>,
    tax_conditions: TaxConditionsDetails,
    payment_conditions: PaymentConditionsDetails,
    title: String,
    introduction: String,
    remark: String,
    files: FilesDetails,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddressDetails {
    contact_id: Uuid,
    name: String,
    supplement: String,
    street: String,
    city: String,
    zip: String,
    country_code: Country,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItemsDetails {
    id: Option<Uuid>,
    r#type: TypeEnum,
    name: String,
    description: String,
    quantity: f64,
    unit_name: String,
    unit_price: UnitPriceDetails,
    discount_percentage: f64,
    line_item_amount: f64,
    sub_items: Vec<LineItemsDetails>,
    optional: bool,
    alternative: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPriceDetails {
    currency: CurrencyEnum,
    net_amount: f64,
    gross_amount: f64,
    tax_rate_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPriceDetails {
    currency: String,
    total_net_amount: f64,
    total_gross_amount: f64,
    total_tax_amount: f64,
    total_discount_absolute: f64,
    total_discount_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxAmountsDetails {
    tax_rate_percentage: f64,
    tax_amount: f64,
    net_amount: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditionsDetails {
    tax_type: TaxTypeEnum,
    tax_type_note: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentConditionsDetails {
    payment_term_label: String,
    payment_term_duration: i64,
    payment_discount_conditions: Vec<PaymentDiscountConditionsDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentDiscountConditionsDetails {
    discount_percentage: f64,
    discount_range: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FilesDetails {
    document_file_id: Uuid,
}
