#![doc = "This endpoint provides read and write access to quotations which can be created as draft or finalized in open mode and additionally downloaded as printed pdf document.\n\nPlease note that Public API connections that were established prior to the release of the quotations endpoint (see [Change Log](#change-log)) are not automatically granted the permission for quotations access. Re\\-generate a new Public API key to benefit from quotations access.\n\nIt is possible to create quotations with value\\-added tax such as of type net (*Netto*), gross (*Brutto*) or different types of vat\\-free. For tax\\-exempt organizations vat\\-free (*Steuerfrei*) quotations can be created exclusively. All other vat\\-free tax types are only usable in combination with a referenced contact in lexoffice. For recipients within the EU these are intra\\-community supply (*Innergemeinschaftliche Lieferung gem. §13b UStG*), constructional services (*Bauleistungen gem. §13b UStG*) and external services (*Fremdleistungen innerhalb der EU gem. §13b UStG*). For quotations to third countries, the tax types third party country service (*Dienstleistungen an Drittländer*) and third party country delivery (*Ausfuhrlieferungen an Drittländer*) are possible."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
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
pub enum TaxType {
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "vatfree")]
    Vatfree,
    #[serde(rename = "intraCommunitySupply")]
    IntraCommunitySupply,
    #[serde(rename = "constructionService13b")]
    ConstructionService13b,
    #[serde(rename = "externalService13b")]
    ExternalService13b,
    #[serde(rename = "thirdPartyCountryService")]
    ThirdPartyCountryService,
    #[serde(rename = "thirdPartyCountryDelivery")]
    ThirdPartyCountryDelivery,
}
impl std::str::FromStr for TaxType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Type {
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "material")]
    Material,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "text")]
    Text,
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
pub struct Quotation {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the quotation belongs to.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the quotation was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the quotation was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the quotation will expire. Value in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[builder(setter(into))]
    pub expiration_date: crate::types::DateTime,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the quotation which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639\\-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "Specifies if the quotation is only available in the archive in lexoffice.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub archived: crate::marker::ReadOnly<bool>,
    #[doc = "Specifies the status of the quotation. Possible values are **draft** (editable later in lexoffice), **open** (finished and no longer editable but yet neither accepted nor rejected), **accepted** (has been accepted by the customer), **rejected** (rejected by the customer)  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_status: crate::marker::ReadOnly<VoucherStatus>,
    #[doc = "The specific number a quotation is aware of. This consecutive number is set by lexoffice on creation.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_number: crate::marker::ReadOnly<String>,
    #[doc = "The date of quotation in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[builder(setter(into))]
    pub voucher_date: crate::types::DateTime,
    #[doc = "The address of the quotation recipient. For details see below."]
    #[builder(setter(into))]
    pub address: Address,
    #[doc = "The items of the quotation. For details see below."]
    #[builder(setter(into))]
    pub line_items: Vec<LineItems>,
    #[doc = "The total price of the quotation. For details see below."]
    #[builder(setter(into))]
    pub total_price: TotalPrice,
    #[doc = "The tax amounts for each tax rate. Please note: As done with every read\\-only element or object all submitted content (POST) will be ignored. For details see below.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub tax_amounts: crate::marker::ReadOnly<Vec<TaxAmounts>>,
    #[doc = "The tax conditions of the quotation. For details see below."]
    #[builder(setter(into))]
    pub tax_conditions: TaxConditions,
    #[doc = "The payment conditions of the quotation. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_conditions: Option<PaymentConditions>,
    #[doc = "(Optional) A title text. The organization's default is used if no value was sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
    #[doc = "(Optional) An introductory text / header. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub introduction: Option<String>,
    #[doc = "(Optional) A closing text note. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remark: Option<String>,
    #[doc = "The document id for the PDF version of the quotation. For details see below.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub files: crate::marker::ReadOnly<Files>,
}
impl crate::request::HasId for Quotation {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "There are two main options to address the recipient of a quotation. First, using an existing lexoffice contact or second creating a new address.\n\nFor **referencing an existing contact** it is only necessary to provide the UUID of that contact. Additionally, the referenced address can also be modified for this specific quotation. Therefore all required address fields must be set and the deviated address will not be stored back to the lexoffice contacts.\n\nThe referenced contact needs to have the role customer. For more information please refer to the Contacts Endpoint.\n\nOtherwise, a **new address** for the quotation recipient can be created. That type of address is called a \"one\\-time address\". A one\\-time address will not create a new contact in lexoffice. For instance, this could be useful when it is not needed to create a contact in lexoffice for each new quotation.\n\nPlease get in touch with us if you are not sure which option fits your use case best."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "If the quotation recipient is (optionally) registered as a contact in lexoffice, this field specifies the related id of the contact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "The name of the quotation recipient. To use an existing contact of an individual person, provide the name in the format {firstname} {lastname}."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[doc = "(Optional) An address supplement."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub supplement: Option<String>,
    #[doc = "The street (street and street number) of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub street: Option<String>,
    #[doc = "The city of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub city: Option<String>,
    #[doc = "The zip code of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub zip: Option<String>,
    #[doc = "The ISO 3166 alpha2 country code of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_code: Option<crate::types::CountryCode>,
    #[doc = "The contact person selected while editing the voucher. The primary contact person will be used when creating vouchers via the API with a referenced `contactId`.  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub contact_person: crate::marker::ReadOnly<String>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItems {
    #[doc = "The field specifies the related id of the product/service.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The type of the item. Possible values are **service** (the line item is related to a supply of services), **material** (the line item is related to a physical product), **custom** (an item without reference in lexoffice and has no id) or **text** (contains only a name and/or a description for informative purposes)."]
    #[builder(setter(into))]
    pub _type: Type,
    #[doc = "The name of the item."]
    #[builder(setter(into))]
    pub name: String,
    #[doc = "The description of the item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[doc = "The amount of the purchased item. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub quantity: Option<f64>,
    #[doc = "The unit name of the purchased item. If the provided unit name is not known in lexoffice it will be created on the fly."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub unit_name: Option<String>,
    #[doc = "The unit price of the purchased item. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub unit_price: Option<UnitPrice>,
    #[doc = "The offered discount for the item. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_percentage: Option<f64>,
    #[doc = "The total price of this line item. Depending by the selected *taxType* in *taxConditions*, the amount must be given either as net or gross. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub line_item_amount: crate::marker::ReadOnly<f64>,
    #[doc = "A list of subitems of this line item. At this time, all `subItems` need to be alternative items."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sub_items: Option<Vec<LineItems>>,
    #[doc = "If true, the line item is optional (\"Optionale Position\"). Not a valid attribute for subitems. Defaults to false if unset"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub optional: Option<bool>,
    #[doc = "If true, the line item is an alternative position for its parent item. Currently only valid for subitems, and mandatory to be true in that case. Defaults to false if unset"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alternative: Option<bool>,
}
impl crate::request::HasId for LineItems {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPrice {
    #[doc = "The currency of the price. Currently only **EUR** is supported."]
    #[builder(setter(into))]
    pub currency: crate::types::Currency,
    #[doc = "The net price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
    #[doc = "The gross price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gross_amount: Option<f64>,
    #[doc = "The tax rate of the unit price. [Supported tax rates](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19**. For vat\\-free sales vouchers the tax rate percentage must be **0**."]
    #[builder(setter(into))]
    pub tax_rate_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPrice {
    #[doc = "The currency of the total price. Currently only **EUR** is supported."]
    #[builder(setter(into))]
    pub currency: crate::types::Currency,
    #[doc = "The total net price over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub total_net_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total gross price over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub total_gross_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total tax amount over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub total_tax_amount: crate::marker::ReadOnly<f64>,
    #[doc = "(Optional) A total discount as absolute value. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_discount_absolute: Option<f64>,
    #[doc = "(Optional) A total discount relative to the gross amount or net amount dependent on the given tax conditions. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_discount_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxAmounts {
    #[doc = "Tax rate as percentage value. [Supported tax rates](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_rate_percentage: Option<f64>,
    #[doc = "The total tax amount for this tax rate. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_amount: Option<f64>,
    #[doc = "The total net amount for this tax rate. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditions {
    #[doc = "The tax type for the quotation. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. §13b UStG*), **constructionService13b** (*Bauleistungen gem. §13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. §13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittländer*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittländer*)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type: Option<TaxType>,
    #[doc = "When *taxType* is set to a vat\\-free tax type then a note regarding the conditions can be set. When omitted lexoffice sets the organization's default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type_note: Option<String>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentConditions {
    #[doc = "A textual note regarding the payment conditions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_label: Option<String>,
    #[doc = "The time left (in days) until the payment must be conducted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_duration: Option<i64>,
    #[doc = "The payment discount conditions for the quotation."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_discount_conditions: Option<Vec<PaymentDiscountConditions>>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentDiscountConditions {
    #[doc = "The discount offered in return for payment within the **discountRange**. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_percentage: Option<f64>,
    #[doc = "The time left (in days) the discount is valid."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub discount_range: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Files {
    #[doc = "The id of the quotation PDF. The PDF will be created when the quotation turns from **draft** into status **open**. To download the quotation PDF file please use the files endpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub document_file_id: Option<uuid::Uuid>,
}
