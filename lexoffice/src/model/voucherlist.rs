#![doc = "The voucherlist endpoint provides read access to meta data of [invoices](#invoices-endpoint), [credit notes](#credit-notes-endpoint), [order confirmations](#order-confirmations-endpoint), and [quotations](#quotations-endpoint). For filters that can be applied to the list see below. Details concerning items from the list are accessible by id using the respective endpoint. For more information on the different voucher types refer to the documentation on the respective endpoints."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "paidoff")]
    Paidoff,
    #[serde(rename = "voided")]
    Voided,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}
impl std::str::FromStr for VoucherStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherType {
    #[serde(rename = "salesinvoice")]
    Salesinvoice,
    #[serde(rename = "salescreditnote")]
    Salescreditnote,
    #[serde(rename = "purchaseinvoice")]
    Purchaseinvoice,
    #[serde(rename = "purchasecreditnote")]
    Purchasecreditnote,
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "creditnote")]
    Creditnote,
    #[serde(rename = "orderconfirmation")]
    Orderconfirmation,
    #[serde(rename = "quotation")]
    Quotation,
}
impl std::str::FromStr for VoucherType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[doc = "This section describes the properties of the meta data object for vouchers returned by this endpoint."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Voucherlist {
    #[doc = "Unique id of the voucher in lexoffice."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Type of the voucher. Possible values are **salesinvoice**, **salescreditnote**, **purchaseinvoice**, **purchasecreditnote**, **invoice**, **creditnote**, **orderconfirmation**, and **quotation**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_type: Option<VoucherType>,
    #[doc = "Showing the current workflow status of the voucher in lexoffice. Possible values are **draft**, **open**, **paid**, **paidoff**, **voided**, **overdue**, **accepted**, and **rejected**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_status: Option<VoucherStatus>,
    #[doc = "Identification/Reference number of the voucher."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_number: Option<String>,
    #[doc = "Date when the voucher was issued. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_date: Option<crate::types::DateTime>,
    #[doc = "Date when the voucher was last changed (or status changed) in lexoffice. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub updated_date: Option<crate::types::DateTime>,
    #[doc = "Date when the voucher's payment has to be settled. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub due_date: Option<crate::types::DateTime>,
    #[doc = "Name of the recipient or invoicing party. Further information on the contact can be retrieved by using the corresponding voucher endpoint (see below)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_name: Option<String>,
    #[doc = "Total amount of the voucher (may include taxes). Format is **##.00** *(119.00)*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_amount: Option<f64>,
    #[doc = "Currency of the voucher. Only possible value is **EUR**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
    #[doc = "Indicates if the voucher is marked as archived in lexoffice."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub archived: Option<bool>,
}
impl crate::request::HasId for Voucherlist {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
