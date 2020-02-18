use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum CurrencyEnum {
    EUR,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum VoucherStatusEnum {
    Draft,
    Open,
    Paid,
    Paidoff,
    Voided,
    Overdue,
    Accepted,
    Rejected,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum VoucherTypeEnum {
    Invoice,
    Creditnote,
    Orderconfirmation,
    Quotation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VoucherList {
    pub voucher_type: VoucherTypeEnum,
    pub voucher_status: VoucherStatusEnum,
    pub voucher_number: String,
    pub voucher_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub contact_name: String,
    pub total_amount: f64,
    pub currency: CurrencyEnum,
    pub archived: bool,
}
