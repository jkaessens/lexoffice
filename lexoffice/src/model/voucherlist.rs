#![doc = "The voucherlist endpoint provides read access to meta data of (bookkeeping) [vouchers](#vouchers-endpoint) (e.g. salesinvoices, salescreditnotes), [invoices](#invoices-endpoint) (including [down payment invoices](#down-payment-invoices-endpoint)), [credit notes](#credit-notes-endpoint), [order confirmations](#order-confirmations-endpoint), [quotations](#quotations-endpoint), and [delivery notes](#delivery-notes-endpoint). Details concerning items from the list are accessible by id using the respective endpoint. For more information on the different voucher types refer to the documentation on the respective endpoints. The voucherlist can be searched using various filters to get only the data you need."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "paidoff")]
    Paidoff,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "sepadebit")]
    Sepadebit,
    #[serde(rename = "transferred")]
    Transferred,
    #[serde(rename = "voided")]
    Voided,
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
    #[serde(rename = "creditnote")]
    Creditnote,
    #[serde(rename = "deliverynote")]
    Deliverynote,
    #[serde(rename = "downpaymentinvoice")]
    Downpaymentinvoice,
    #[serde(rename = "invoice")]
    Invoice,
    #[serde(rename = "orderconfirmation")]
    Orderconfirmation,
    #[serde(rename = "purchasecreditnote")]
    Purchasecreditnote,
    #[serde(rename = "purchaseinvoice")]
    Purchaseinvoice,
    #[serde(rename = "quotation")]
    Quotation,
    #[serde(rename = "salescreditnote")]
    Salescreditnote,
    #[serde(rename = "salesinvoice")]
    Salesinvoice,
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
    #[doc = "Type of the voucher. Possible values are **salesinvoice**, **salescreditnote**, **purchaseinvoice**, **purchasecreditnote**, **invoice**, **downpaymentinvoice**, **creditnote**, **orderconfirmation**, **quotation**, and **deliverynote**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_type: Option<VoucherType>,
    #[doc = "Showing the current workflow status of the voucher in lexoffice. Possible values are **draft**, **open**, **paid**, **paidoff**, **voided**, **transferred**, **sepadebit**, **overdue**, **accepted**, and **rejected**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_status: Option<VoucherStatus>,
    #[doc = "Identification/Reference number of the voucher."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_number: Option<String>,
    #[doc = "Date when the voucher was issued. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_date: Option<crate::types::DateTime>,
    #[doc = "Date when the voucher was created in lexoffice. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub created_date: Option<crate::types::DateTime>,
    #[doc = "Date when the voucher was last changed (or status changed) in lexoffice. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub updated_date: Option<crate::types::DateTime>,
    #[doc = "Date when the voucher's payment has to be settled. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub due_date: Option<crate::types::DateTime>,
    #[doc = "The id of an existing contact in lexoffice which is the recipient or invoicing party. Will be null for the [Collective Contact](https://developers.lexoffice.io/docs/#faq)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "Name of the recipient or invoicing party."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_name: Option<String>,
    #[doc = "Total amount of the voucher (may include taxes). Format is **##.00** *(119.00)*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_amount: Option<f64>,
    #[doc = "Open amount of the voucher. May be null (e.g., for invoices in draft, or various non-invoice vouchers). Format is **##.00** *(123.00)*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub open_amount: Option<f64>,
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
