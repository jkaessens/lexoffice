use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CurrencyEnum {
    EUR,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum VoucherStatusEnum {
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
#[serde(rename_all = "camelCase")]
enum VoucherTypeEnum {
    Invoice,
    Creditnote,
    Orderconfirmation,
    Quotation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voucherlist {
    id: Option<Uuid>,
    voucher_type: VoucherTypeEnum,
    voucher_status: VoucherStatusEnum,
    voucher_number: String,
    voucher_date: DateTime<Utc>,
    updated_date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    contact_name: String,
    total_amount: f64,
    currency: CurrencyEnum,
    archived: bool,
}
