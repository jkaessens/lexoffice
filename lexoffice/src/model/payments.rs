#![doc = "The payments endpoint provides read access to the payment status of (bookkeeping or sales) vouchers, including invoices and credit notes."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PaymentStatus {}
impl std::str::FromStr for PaymentStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {}
impl std::str::FromStr for VoucherStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherType {}
impl std::str::FromStr for VoucherType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Payment {
    #[doc = "Open amount. Positive value both for revenues and expenses"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub open_amount: Option<f64>,
    #[doc = "Always contains the value **EUR**, the only currently supported currency"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
    #[doc = "The payment status is one of the values *balanced*, *openRevenue*, or *openExpense*"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_status: Option<PaymentStatus>,
    #[doc = "Contains the voucher type: *salesinvoice*, *salescreditnote*, *purchaseinvoice*, *purchasecreditnote*, *invoice*, *downpaymentinvoice*, *creditnote*"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_type: Option<VoucherType>,
    #[doc = "Contains one of the following voucher states: *open*, *paid*, *paidoff*, *voided*, *transferred*, *sepadebit*"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_status: Option<VoucherStatus>,
}
