# ! [ doc = "With a credit note the partial or full amount of an invoice can be refunded to a customer.\n\nAt the moment, credit notes can be created without any reference to an invoice. To refer a credit note to an invoice on the printed document, the invoice number can be included in the header text (*introduction*).\n\nIt is possible to create credit notes with value\\-added tax such as of type net (*Netto*), gross (*Brutto*) or different types of vat\\-free. For tax\\-exempt organizations vat\\-free (*Steuerfrei*) credit notes can be created exclusively. All other vat\\-free tax types are only usable in combination with a referenced contact in lexoffice. For recipients within the EU these are intra\\-community supply (*Innergemeinschaftliche Lieferung gem. \u{a7}13b UStG*), constructional services (*Bauleistungen gem. \u{a7}13b UStG*) and external services (*Fremdleistungen innerhalb der EU gem. \u{a7}13b UStG*). For credit notes to third countries, the tax types third party country service (*Dienstleistungen an Drittl\u{e4}nder*) and third party country delivery (*Ausfuhrlieferungen an Drittl\u{e4}nder*) are possible." ]use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paidoff")]
    Paidoff,
    #[serde(rename = "voided")]
    Voided,
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
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreditNote {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub id: super::super::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the credit note belongs to.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub organization_id: super::super::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the credit note was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub created_date:
        super::super::marker::ReadOnly<chrono::DateTime<chrono::Utc>>,
    #[doc = "The instant of time when the credit note was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub updated_date:
        super::super::marker::ReadOnly<chrono::DateTime<chrono::Utc>>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](#optimistic-locking).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the credit note which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639\\-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "Specifies if the credit note is only available in the archive in lexoffice.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub archived: super::super::marker::ReadOnly<bool>,
    #[doc = "Specifies the status of the credit note. Possible values are **draft** (editable later in lexoffice), **open** (finished and no longer editable but not yet paid off), **paidoff** (has been fully paid back to the customer), **voided** (cancelled)  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub voucher_status: super::super::marker::ReadOnly<VoucherStatus>,
    #[doc = "The specific number a credit note is aware of. This consecutive number is set by lexoffice on creation.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub voucher_number: super::super::marker::ReadOnly<String>,
    #[doc = "The date of credit note in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*)."]
    #[builder(setter(into))]
    pub voucher_date: chrono::DateTime<chrono::Utc>,
    #[doc = "The address of the credit note recipient. For details see below."]
    #[builder(setter(into))]
    pub address: Address,
    #[doc = "The items of the credit note. For details see below."]
    #[builder(setter(into))]
    pub line_items: Vec<LineItems>,
    #[doc = "The total price of the credit note. For details see below."]
    #[builder(setter(into))]
    pub total_price: TotalPrice,
    #[doc = "The tax amounts for each tax rate. Please note: As done with every read\\-only element or object all submitted content (POST) will be ignored. For details see below.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub tax_amounts: super::super::marker::ReadOnly<Vec<TaxAmounts>>,
    #[doc = "The tax conditions of the credit note. For details see below."]
    #[builder(setter(into))]
    pub tax_conditions: TaxConditions,
    #[doc = "(Optional) A title text. The organization's default is used if no value was sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
    #[doc = "(Optional) An introductory text / header. The organization's default is used if no value was send. **We recommended to include the invoice number in the header when the credit note is related to an invoice.**"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub introduction: Option<String>,
    #[doc = "(Optional) A closing text note. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remark: Option<String>,
    #[doc = "The document id for the PDF version of the credit note. For details see below.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub files: super::super::marker::ReadOnly<uuid::Uuid>,
}
impl super::super::request::HasId for CreditNote {
    fn id(&self) -> &super::super::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "There are two main options to address the recipient of a credit note. First, using an existing lexoffice contact or second creating a new address.\n\nFor **referencing an existing contact** it is only necessary to provide the UUID of that contact. Additionally, the referenced address can also be modified for this specific credit note. Therefore all required address fields must be set and the deviated address will not be stored back to the lexoffice contacts.\n\nThe referenced contact needs to have the role customer. For more information please refer to the Contacts Endpoint.\n\nOtherwise, a **new address** for the credit note recipient can be created. That type of address is called a \"one\\-time address\". A one\\-time address will not create a new contact in lexoffice. For instance, this could be useful when it is not needed to create a contact in lexoffice for each new credit note.\n\nPlease get in touch with us if you are not sure which option fits your use case best."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "If the credit note recipient is (optionally) registered as a contact in lexoffice, this field specifies the related id of the contact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "The name of the credit note recipient. To use an existing contact of an individual person, provide the name in the format {firstname} {lastname}."]
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
    pub country_code: Option<celes::Country>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItems {
    #[doc = "The field specifies the related id of the product/service.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub id: super::super::marker::ReadOnly<uuid::Uuid>,
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
    #[doc = "The total price of this line item. Depending by the selected *taxType* in *taxConditions*, the amount must be given either as net or gross. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub line_item_amount: super::super::marker::ReadOnly<f64>,
}
impl super::super::request::HasId for LineItems {
    fn id(&self) -> &super::super::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnitPrice {
    #[doc = "The currency of the price. Currently only **EUR** is supported."]
    #[builder(setter(into))]
    pub currency: iso_currency::Currency,
    #[doc = "The net price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
    #[doc = "The gross price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gross_amount: Option<f64>,
    #[doc = "The tax rate of the unit price. [Supported tax rates](#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19**. For vat\\-free sales vouchers the tax rate percentage must be **0**."]
    #[builder(setter(into))]
    pub tax_rate_percentage: f64,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPrice {
    #[doc = "The currency of the total price. Currently only **EUR** is supported."]
    #[builder(setter(into))]
    pub currency: iso_currency::Currency,
    #[doc = "The total net price over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub total_net_amount: super::super::marker::ReadOnly<f64>,
    #[doc = "The total gross price over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub total_gross_amount: super::super::marker::ReadOnly<f64>,
    #[doc = "The total tax amount over all line items. The value can contain up to 2 decimals.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    #[serde(skip_serializing)]
    pub total_tax_amount: super::super::marker::ReadOnly<f64>,
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
    #[doc = "Tax rate as percentage value. [Supported tax rates](#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19**."]
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
    #[doc = "The tax type for the credit note. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. \u{a7}13b UStG*), **constructionService13b** (*Bauleistungen gem. \u{a7}13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. \u{a7}13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittl\u{e4}nder*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittl\u{e4}nder*)"]
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
pub struct Files {
    #[doc = "The id of the credit note PDF. The PDF will be created when the credit note turns from **draft** into status **open** or **paidoff**. To download the credit note PDF file please use the files endpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub document_file_id: Option<uuid::Uuid>,
}
