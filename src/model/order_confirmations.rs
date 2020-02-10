use celes::Country;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum CurrencyEnum {
    EUR,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum ShippingTypeEnum {
    Service,
    Serviceperiod,
    Delivery,
    Deliveryperiod,
    None,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum TaxTypeEnum {
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
pub enum TypeEnum {
    Service,
    Material,
    Custom,
    Text,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum VoucherStatusEnum {
    Draft,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OrderConfirmation {
    pub organization_id: Uuid,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub language: String,
    pub archived: bool,
    pub voucher_status: VoucherStatusEnum,
    pub voucher_number: String,
    pub voucher_date: DateTime<Utc>,
    pub address: AddressDetails,
    pub line_items: Vec<LineItemsDetails>,
    pub total_price: TotalPriceDetails,
    pub tax_amounts: Vec<TaxAmountsDetails>,
    pub tax_conditions: TaxConditionsDetails,
    pub payment_conditions: PaymentConditionsDetails,
    pub shipping_conditions: ShippingConditionsDetails,
    pub title: String,
    pub introduction: String,
    pub remark: String,
    pub delivery_terms: String,
    pub files: FilesDetails,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AddressDetails {
    pub contact_id: Uuid,
    pub name: String,
    pub supplement: String,
    pub street: String,
    pub city: String,
    pub zip: String,
    pub country_code: Country,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItemsDetails {
    pub id: Option<Uuid>,
    pub r#type: TypeEnum,
    pub name: String,
    pub description: String,
    pub quantity: f64,
    pub unit_name: String,
    pub unit_price: UnitPriceDetails,
    pub discount_percentage: f64,
    pub line_item_amount: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPriceDetails {
    pub currency: CurrencyEnum,
    pub net_amount: f64,
    pub gross_amount: f64,
    pub tax_rate_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPriceDetails {
    pub currency: String,
    pub total_net_amount: f64,
    pub total_gross_amount: f64,
    pub total_tax_amount: f64,
    pub total_discount_absolute: f64,
    pub total_discount_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxAmountsDetails {
    pub tax_rate_percentage: f64,
    pub tax_amount: f64,
    pub net_amount: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditionsDetails {
    pub tax_type: TaxTypeEnum,
    pub tax_type_note: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentConditionsDetails {
    pub payment_term_label: String,
    pub payment_term_duration: i64,
    pub payment_discount_conditions: Vec<PaymentDiscountConditionsDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentDiscountConditionsDetails {
    pub discount_percentage: f64,
    pub discount_range: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShippingConditionsDetails {
    pub shipping_date: DateTime<Utc>,
    pub shipping_end_date: DateTime<Utc>,
    pub shipping_type: ShippingTypeEnum,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FilesDetails {
    pub document_file_id: Uuid,
}
