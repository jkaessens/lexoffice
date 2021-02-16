#![doc = "The voucher endpoint provides read/write access to (bookkeeping) sales vouchers (e.g. invoices, creditnotes). For the lexoffice API, write access is currently restricted to sales vouchers, while purchase vouchers can be read.\n\nIn contrast to usual invoices which contain item positions, stocks, etc., bookkeeping vouchers are more about bookkeeping and thus this only have positions grouped by tax rate. Optionally, a file (pdf or image) can be added to the voucher as a *receipt* which was created by an external system.\n\nA higher level description of the handling of vouchers via the lexoffice API can be found in the [bookkeeping cookbook](../cookbooks/bookkeeping/) (German only)."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxType {
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "gross")]
    Gross,
}
impl std::str::FromStr for TaxType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "paidoff")]
    Paidoff,
    #[serde(rename = "voided")]
    Voided,
    #[serde(rename = "transferred")]
    Transferred,
    #[serde(rename = "sepadebit")]
    Sepadebit,
}
impl std::str::FromStr for VoucherStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Type {
    #[serde(rename = "salesinvoice")]
    Salesinvoice,
    #[serde(rename = "salescreditnote")]
    Salescreditnote,
    #[serde(rename = "purchaseinvoice")]
    Purchaseinvoice,
    #[serde(rename = "purchasecreditnote")]
    Purchasecreditnote,
}
impl std::str::FromStr for Type {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Voucher {
    #[doc = "Unique id of the voucher generated on creation by lexoffice."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the voucher was generated on."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_id: Option<uuid::Uuid>,
    #[doc = "Type of the voucher. Possible values are **salesinvoice** (e.g. for sales orders), **salescreditnote** (e.g. for refunds or returned sales orders), **purchaseinvoice** and **purchasecreditnote**. Note that the same categoryId can be used for salescreditnotes and for salesinvoices."]
    #[builder(setter(into))]
    pub _type: Type,
    #[doc = "Billing state of the voucher. Possible values are **open**, **paid**, **paidoff**, **voided**, **transferred** and **sepadebit**.  \n*Read-only*."]
    #[builder(default, setter(skip))]
    pub voucher_status: crate::marker::ReadOnly<VoucherStatus>,
    #[doc = "Number of the voucher. Should be the order's identification/reference number."]
    #[builder(setter(into))]
    pub voucher_number: String,
    #[doc = "Date when the voucher was issued. Format must be `yyyy-MM-dd` (e.g. *2016-06-28*)."]
    #[serde(with = "crate::serde::date")]
    #[builder(setter(into))]
    pub voucher_date: crate::types::Date,
    #[doc = "Date when the purchased item/service has to be shipped/supplied. If it is a period of time, the end date must be given. Format must be `yyyy-MM-dd` (e.g. *2016-07-02*). Please note: ShippingDate can only be specified for voucher types **salesinvoice** and **salescreditnote**."]
    #[serde(with = "crate::serde::optional_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_date: Option<crate::types::Date>,
    #[doc = "Date when the voucher's payment has to be settled. Format must be `yyyy-MM-dd` (e.g. *2016-06-28*)."]
    #[serde(with = "crate::serde::optional_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub due_date: Option<crate::types::Date>,
    #[doc = "Total gross amount of the voucher. Must match the sum of all positions with added/calculated tax amounts. Format must be **##.00** *(119.00)*."]
    #[builder(setter(into))]
    pub total_gross_amount: f64,
    #[doc = "Total tax amount of the voucher. Must match the sum of all positions' tax amounts. Format must be **##.00** *(19.00)*."]
    #[builder(setter(into))]
    pub total_tax_amount: f64,
    #[doc = "Tax type of the order. Possible values are **net** (positions amount will be provided net and taxes have to be calculated on top), **gross** (positions amount will be provided gross and tax is already included)."]
    #[builder(setter(into))]
    pub tax_type: TaxType,
    #[doc = "Set to true if the [Collective Contact](https://developers.lexoffice.io/docs/#faq) (customer/vendor) within lexoffice should be used. If used, the optional contactId will be ignored."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_collective_contact: Option<bool>,
    #[doc = "If not using the collective contact option, an existing contact id must be provided. This must exist within lexoffice before and can be created via the [Contacts](https://developers.lexoffice.io/docs/#contacts-endpoint) endpoint. If a contact is assigned to a voucher, its role must either be **Customer**, or both **Customer** and **Vendor**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "Any comments or remarks to the order. This field is part of the full text search of lexoffice, any information for finding the voucher can be placed here as convenience of the lexoffice user."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remark: Option<String>,
    #[doc = "Positions of the voucher grouped by tax rate. Specification of object voucherItems please see below."]
    #[builder(setter(into))]
    pub voucher_items: Vec<VoucherItems>,
    #[doc = "A list of voucher image file uuids. Voucher images can be uploaded and assigned to an existing voucher via the [Upload a File to a Voucher](https://developers.lexoffice.io/docs/#Upload-a-File-to-a-Voucher) sub-resource endpoint. **Please note: Each file (voucher image) can only be assigned once.**"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub files: Option<Vec<uuid::Uuid>>,
    #[doc = "The instant of time when the voucher was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the voucher was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking). Set to **0 for initial POST**, for **PUT get latest version from lexoffice** *(via GET)* and merge with your changes. **Please note: If the version did not match the version stored in your system, the user must be informed about losing changes from lexoffice.**"]
    #[builder(default, setter(skip))]
    pub version: i64,
}
impl crate::request::HasId for Voucher {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VoucherItems {
    #[doc = "Amount of the position. Net or gross amount, according to the voucher's taxType. Format must be **##.00** *(119.00)*."]
    #[builder(setter(into))]
    pub amount: f64,
    #[doc = "Tax amount of the voucher's item. Format must be **##.00** *(19.00)*."]
    #[builder(setter(into))]
    pub tax_amount: f64,
    #[doc = "Tax rate as percentage value. [Supported tax rates](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19** (*e.g. 19*)."]
    #[builder(setter(into))]
    pub tax_rate_percent: f64,
    #[doc = "Booking category for this voucher's revenue or expenditure. Supported and appropriate categoryId's can be found [here](https://developers.lexoffice.io/docs/#vouchers-endpoint-list-of-categoryids)."]
    #[builder(setter(into))]
    pub category_id: uuid::Uuid,
}
