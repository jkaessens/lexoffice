#![doc = "This endpoint provides read and write access to credit notes and also the possibility to render the document as a PDF in order to download it. Credit notes can be created as a draft or finalized in open mode.\n\nWith a credit note the partial or full amount of an invoice can be refunded to a customer.\n\nAt the moment, credit notes can only be created without any reference to an invoice. To refer a credit note to an invoice on the printed document, the invoice number can be included in the header text (*introduction*).\n\nIt is possible to create credit notes with value-added tax such as of type net (*Netto*), gross (*Brutto*) or different types of vat-free. For tax-exempt organizations vat-free (*Steuerfrei*) credit notes can be created exclusively. All other vat-free tax types are only usable in combination with a referenced contact in lexoffice. For recipients within the EU these are intra-community supply (*Innergemeinschaftliche Lieferung gem. §13b UStG*), constructional services (*Bauleistungen gem. §13b UStG*) and external services (*Fremdleistungen innerhalb der EU gem. §13b UStG*). For credit notes to third countries, the tax types third party country service (*Dienstleistungen an Drittländer*) and third party country delivery (*Ausfuhrlieferungen an Drittländer*) are possible."]
use serde::{Deserialize, Serialize};
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
#[doc = "```json\n{\n    \"id\": \"e9066f04-8cc7-4616-93f8-ac9ecc8479c8\",\n    \"organizationId\": \"aa93e8a8-2aa3-470b-b914-caad8a255dd8\",\n    \"createdDate\": \"2019-06-17T18:32:07.480+02:00\",\n    \"updatedDate\": \"2019-06-17T18:32:07.551+02:00\",\n    \"version\": 1,\n    \"language\": \"de\",\n    \"archived\": false,\n    \"voucherStatus\": \"draft\",\n    \"voucherNumber\": \"GS0007\",\n    \"voucherDate\": \"2017-02-22T00:00:00.000+01:00\",\n    \"address\": {\n        \"name\": \"Bike & Ride GmbH & Co. KG\",\n        \"supplement\": \"Gebäude 10\",\n        \"street\": \"Musterstraße 42\",\n        \"city\": \"Freiburg\",\n        \"zip\": \"79112\",\n        \"countryCode\": \"DE\"\n    },\n    \"lineItems\": [\n        {\n            \"type\": \"custom\",\n            \"name\": \"Abus Kabelschloss Primo 590 \",\n            \"description\": \"· 9,5 mm starkes, smoke-mattes Spiralkabel mit integrierter Halterlösung zur Befestigung am Sattelklemmbolzen · bewährter Qualitäts-Schließzylinder mit praktischem Wendeschlüssel · KabelØ: 9,5 mm, Länge: 150 cm\",\n            \"quantity\": 2,\n            \"unitName\": \"Stück\",\n            \"unitPrice\": {\n                \"currency\": \"EUR\",\n                \"netAmount\": 13.4,\n                \"grossAmount\": 15.946,\n                \"taxRatePercentage\": 19\n            },\n            \"lineItemAmount\": 26.8\n        },\n        {\n            \"type\": \"custom\",\n            \"name\": \"Energieriegel Testpaket\",\n            \"quantity\": 1,\n            \"unitName\": \"Stück\",\n            \"unitPrice\": {\n                \"currency\": \"EUR\",\n                \"netAmount\": 5,\n                \"grossAmount\": 5,\n                \"taxRatePercentage\": 0\n            },\n            \"lineItemAmount\": 5\n        }\n    ],\n    \"totalPrice\": {\n        \"currency\": \"EUR\",\n        \"totalNetAmount\": 31.8,\n        \"totalGrossAmount\": 36.89,\n        \"totalTaxAmount\": 5.09\n    },\n    \"taxAmounts\": [\n        {\n            \"taxRatePercentage\": 0,\n            \"taxAmount\": 0,\n            \"netAmount\": 5\n        },\n        {\n            \"taxRatePercentage\": 19,\n            \"taxAmount\": 5.09,\n            \"netAmount\": 26.8\n        }\n    ],\n    \"taxConditions\": {\n        \"taxType\": \"net\"\n    },\n    \"title\": \"Rechnungskorrektur\",\n    \"introduction\": \"Rechnungskorrektur zur Rechnung RE-00020\",\n    \"remark\": \"Folgende Lieferungen/Leistungen schreiben wir Ihnen gut.\",\n    \"files\": {\n      \"documentFileId\": \"a79fea19-a892-4ea9-89ad-e879946329a3\"\n    }\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreditNote {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the credit note belongs to.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the credit note was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the credit note was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the credit note which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "Specifies if the credit note is only available in the archive in lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub archived: crate::marker::ReadOnly<bool>,
    #[doc = "Specifies the status of the credit note. Possible values are **draft** (is editable), **open** (finalized and no longer editable but not yet paid off), **paidoff** (has been fully paid back to the customer), **voided** (cancelled)  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_status: crate::marker::ReadOnly<VoucherStatus>,
    #[doc = "The specific number a credit note is aware of. This consecutive number is set by lexoffice on creation.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_number: crate::marker::ReadOnly<String>,
    #[doc = "The date of credit note in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[builder(setter(into))]
    pub voucher_date: crate::types::DateTime,
    #[doc = "The address of the credit note recipient. For details see below."]
    #[builder(setter(into))]
    pub address: Address,
    #[doc = "The items of the credit note. For details see below."]
    #[builder(setter(into))]
    pub line_items: Vec<LineItems>,
    #[doc = "The total price of the credit note. For details see below."]
    #[builder(setter(into))]
    pub total_price: TotalPrice,
    #[doc = "The tax amounts for each tax rate. Please note: As done with every read-only element or object all submitted content (POST) will be ignored. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub tax_amounts: crate::marker::ReadOnly<Vec<TaxAmounts>>,
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
    #[doc = "The document id for the PDF version of the credit note. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub files: crate::marker::ReadOnly<Files>,
}
impl crate::request::HasId for CreditNote {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "There are two main options to address the recipient of a credit note. First, using an existing lexoffice contact or second creating a new address.\n\nFor **referencing an existing contact** it is only necessary to provide the UUID of that contact. Additionally, the referenced address can also be modified for this specific credit note. Therefore all required address fields must be set and the deviated address will not be stored back to the lexoffice contacts.\n\nThe referenced contact needs to have the role customer. For more information please refer to the Contacts Endpoint.\n\nOtherwise, a **new address** for the credit note recipient can be created. That type of address is called a \"one-time address\". A one-time address will not create a new contact in lexoffice. For instance, this could be useful when it is not needed to create a contact in lexoffice for each new credit note.\n\nPlease get in touch with us if you are not sure which option fits your use case best."]
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
    #[doc = "The [ISO 3166 alpha2 country code](https://developers.lexoffice.io/docs/#faq-country-codes) of the address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub country_code: Option<crate::types::CountryCode>,
    #[doc = "The contact person selected while editing the voucher. The primary contact person will be used when creating vouchers via the API with a referenced `contactId`.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub contact_person: crate::marker::ReadOnly<String>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LineItems {
    #[doc = "The field specifies the related id of the product/service.   \n*Read-only.*"]
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
    #[doc = "The total price of this line item. Depending by the selected *taxType* in *taxConditions*, the amount must be given either as net or gross. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub line_item_amount: crate::marker::ReadOnly<f64>,
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
    #[doc = "The tax rate of the unit price. [Supported tax rates](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) are **0**, **5**, **7**, **16**, **19**. For vat-free sales vouchers the tax rate percentage must be **0**."]
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
    #[doc = "The total net price over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_net_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total gross price over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_gross_amount: crate::marker::ReadOnly<f64>,
    #[doc = "The total tax amount over all line items. The value can contain up to 2 decimals.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub total_tax_amount: crate::marker::ReadOnly<f64>,
    #[doc = "(Optional) A total discount as absolute value. The value can contain up to 2 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_discount_absolute: Option<f64>,
    #[doc = "(Optional) A total discount relative to the gross amount or net amount dependent on the given tax conditions. A contact-specific default will be set if available and no total discount was send. The value can contain up to 2 decimals."]
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
#[doc = "```json\n\"taxConditions\": {\n    \"taxType\": \"constructionService13b\",\n    \"taxTypeNote\": \"Steuerschuldnerschaft des Leistungsempfängers (Reverse Charge)\"\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditions {
    #[doc = "The tax type for the credit note. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. §13b UStG*), **constructionService13b** (*Bauleistungen gem. §13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. §13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittländer*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittländer*)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type: Option<TaxType>,
    #[doc = "When *taxType* is set to a vat-free tax type then a note regarding the conditions can be set. When omitted lexoffice sets the organization's default."]
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
