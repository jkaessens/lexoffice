#![doc = "This endpoint provides read-only access to the templates of recurring invoices, either individually or all as collection. Based on recurring invoice templates, lexoffice will create regular invoices in configured intervals. This operation is executed at night around 2am CET/CEST.\n\nPlease note that it is not possible to query all deduced invoices for a given recurring template. However, when GETting an [invoice](#invoices-endpoint) that was deduced from a recurring template, it will include a reference to the respective recurring template. This allows gathering of information such as the next execution date or the execution status."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ExecutionStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "ENDED")]
    Ended,
    #[serde(rename = "PAUSED")]
    Paused,
}
impl std::str::FromStr for ExecutionStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ExecutionInterval {
    #[serde(rename = "ANNUALLY")]
    Annually,
    #[serde(rename = "BIANNUALLY")]
    Biannually,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    Quarterly,
    #[serde(rename = "WEEKLY")]
    Weekly,
}
impl std::str::FromStr for ExecutionInterval {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxType {
    #[serde(rename = "constructionService13b")]
    ConstructionService13b,
    #[serde(rename = "externalService13b")]
    ExternalService13b,
    #[serde(rename = "gross")]
    Gross,
    #[serde(rename = "intraCommunitySupply")]
    IntraCommunitySupply,
    #[serde(rename = "net")]
    Net,
    #[serde(rename = "thirdPartyCountryDelivery")]
    ThirdPartyCountryDelivery,
    #[serde(rename = "thirdPartyCountryService")]
    ThirdPartyCountryService,
    #[serde(rename = "vatfree")]
    Vatfree,
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
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "material")]
    Material,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "text")]
    Text,
}
impl std::str::FromStr for Type {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ShippingType {
    #[serde(rename = "delivery")]
    Delivery,
    #[serde(rename = "deliveryperiod")]
    Deliveryperiod,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "serviceperiod")]
    Serviceperiod,
}
impl std::str::FromStr for ShippingType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TaxSubType {
    #[serde(rename = "distanceSales")]
    DistanceSales,
    #[serde(rename = "electronicServices")]
    ElectronicServices,
}
impl std::str::FromStr for TaxSubType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[doc = "The set of properties of recurring templates are almost the same as of regular invoices, however, recurring templates\ndo not have any date values set because these will only be derived when the recurring invoices are created.\nAdditionally, the configuration of recurring invoices is defined in a nested object.\nRecurring templates always reference an existing contact.\n\n```json\n{\n  \"id\": \"ac1d66a8-6d59-408b-9413-d56b1db7946f\",\n  \"organizationId\": \"aa93e8a8-2aa3-470b-b914-caad8a255dd8\",\n  \"createdDate\": \"2021-02-10T09:00:00.000+01:00\",\n  \"updatedDate\": \"2021-02-10T09:00:00.000+01:00\",\n  \"version\": 0,\n  \"language\": \"de\",\n  \"archived\": false,\n  \"address\": {\n    \"contactId\": \"464f4881-7a8c-4dc4-87de-7c6fd9a506b8\",\n    \"name\": \"Bike & Ride GmbH & Co. KG\",\n    \"supplement\": \"Gebäude 10\",\n    \"street\": \"Musterstraße 42\",\n    \"city\": \"Freiburg\",\n    \"zip\": \"79112\",\n    \"countryCode\": \"DE\"\n  },\n  \"lineItems\": [\n    {\n      \"id\": \"97b98491-e953-4dc9-97a9-ae437a8052b4\",\n      \"type\": \"material\",\n      \"name\": \"Abus Kabelschloss Primo 590 \",\n      \"description\": \"· 9,5 mm starkes, smoke-mattes Spiralkabel mit integrierter Halterlösung zur Befestigung am Sattelklemmbolzen · bewährter Qualitäts-Schließzylinder mit praktischem Wendeschlüssel · KabelØ: 9,5 mm, Länge: 150 cm\",\n      \"quantity\": 2,\n      \"unitName\": \"Stück\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 13.4,\n        \"grossAmount\": 15.95,\n        \"taxRatePercentage\": 19\n      },\n      \"discountPercentage\": 50,\n      \"lineItemAmount\": 13.4\n    },\n    {\n      \"id\": \"dc4c805b-7df1-4310-a548-22be4499eb04\",\n      \"type\": \"service\",\n      \"name\": \"Aufwändige Montage\",\n      \"description\": \"Aufwand für arbeitsintensive Montagetätigkeit\",\n      \"quantity\": 1,\n      \"unitName\": \"Stunde\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 8.32,\n        \"grossAmount\": 8.9,\n        \"taxRatePercentage\": 7\n      },\n      \"discountPercentage\": 0,\n      \"lineItemAmount\": 8.32\n    },\n    {\n      \"id\": null,\n      \"type\": \"custom\",\n      \"name\": \"Energieriegel Testpaket\",\n      \"description\": null,\n      \"quantity\": 1,\n      \"unitName\": \"Stück\",\n      \"unitPrice\": {\n        \"currency\": \"EUR\",\n        \"netAmount\": 5,\n        \"grossAmount\": 5,\n        \"taxRatePercentage\": 0\n      },\n      \"discountPercentage\": 0,\n      \"lineItemAmount\": 5\n    },\n    {\n      \"type\": \"text\",\n      \"name\": \"Freitextposition\",\n      \"description\": \"This item type can contain either a name or a description or both.\"\n    }\n  ],\n  \"totalPrice\": {\n    \"currency\": \"EUR\",\n    \"totalNetAmount\": 26.72,\n    \"totalGrossAmount\": 29.85,\n    \"totalTaxAmount\": 3.13,\n    \"totalDiscountAbsolute\": null,\n    \"totalDiscountPercentage\": null\n  },\n  \"taxAmounts\": [\n    {\n      \"taxRatePercentage\": 0,\n      \"taxAmount\": 0,\n      \"netAmount\": 5\n    },\n    {\n      \"taxRatePercentage\": 7,\n      \"taxAmount\": 0.58,\n      \"netAmount\": 8.32\n    },\n    {\n      \"taxRatePercentage\": 19,\n      \"taxAmount\": 2.55,\n      \"netAmount\": 13.4\n    }\n  ],\n  \"taxConditions\": {\n    \"taxType\": \"net\",\n    \"taxTypeNote\": null\n  },\n  \"paymentConditions\": {\n    \"paymentTermLabel\": \"10 Tage - 3 %, 30 Tage netto\",\n    \"paymentTermLabelTemplate\": \"{discountRange} Tage -{discount}, {paymentRange} Tage netto\",\n    \"paymentTermDuration\": 30,\n    \"paymentDiscountConditions\": {\n      \"discountPercentage\": 3,\n      \"discountRange\": 10\n    }\n  },\n  \"title\": \"Rechnung\",\n  \"introduction\": \"Ihre bestellten Positionen stellen wir Ihnen hiermit in Rechnung\",\n  \"remark\": \"Vielen Dank für Ihren Einkauf\",\n  \"recurringTemplateSettings\": {\n    \"id\": \"9c5b8bde-7d36-49e8-af5c-4fbe7dc9fa01\",\n    \"startDate\": \"2021-03-01\",\n    \"endDate\": \"2021-06-30\",\n    \"finalize\": true,\n    \"shippingType\": \"service\",\n    \"executionInterval\": \"MONTHLY\",\n    \"nextExecutionDate\": \"2021-03-01\",\n    \"lastExecutionFailed\": false,\n    \"lastExecutionErrorMessage\": null,\n    \"executionStatus\": \"ACTIVE\"\n  }\n}\n\n```\n\nCompared to invoices, recurring templates do not have a voucherStatus, voucherNumber, voucherDate, dueDate, shippingConditions, and files, as these are only derived or calculated during invoice creation."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RecurringTemplate {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the recurring template belongs to.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the invoice was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the invoice was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the invoice which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "The address of the invoice recipient. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub address: Option<Address>,
    #[doc = "The items of the invoice. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub line_items: Option<Vec<LineItems>>,
    #[doc = "The total price of the invoice. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub total_price: Option<TotalPrice>,
    #[doc = "The tax amounts for each tax rate. Please note: As done with every read-only element or object all submitted content (POST) will be ignored. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub tax_amounts: crate::marker::ReadOnly<Vec<TaxAmounts>>,
    #[doc = "The tax conditions of the invoice. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_conditions: Option<TaxConditions>,
    #[doc = "The payment conditions of the invoice. The organization's (or contact-specific) default is used if no value was send. For details see below."]
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
    #[doc = "The settings for creating recurring template.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub recurring_template_settings:
        crate::marker::ReadOnly<RecurringTemplateSettings>,
}
impl crate::request::HasId for RecurringTemplate {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "If the recurring-template recipient is (optionally) registered as a contact in lexoffice, this field specifies the related id of the contact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "The name of the recurring-template recipient. To use an existing contact of an individual person, provide the name in the format {firstname} {lastname}."]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub _type: Option<Type>,
    #[doc = "The name of the item."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
    #[doc = "The net price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub net_amount: Option<f64>,
    #[doc = "The gross price of the unit price. The value can contain up to 4 decimals."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub gross_amount: Option<f64>,
    #[doc = "The tax rate of the unit price. See [the \"Supported tax rates\" FAQ](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) for more information and a list of possible values.. For vat-free sales vouchers the tax rate percentage must be **0**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_rate_percentage: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TotalPrice {
    #[doc = "The currency of the total price. Currently only **EUR** is supported."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub currency: Option<crate::types::Currency>,
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
    #[doc = "Tax rate as percentage value. See [the \"Supported tax rates\" FAQ](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) for more information and a list of possible values.."]
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
    #[doc = "The tax type for the recurring-template. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. §13b UStG*), **constructionService13b** (*Bauleistungen gem. §13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. §13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittländer*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittländer*)"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type: Option<TaxType>,
    #[doc = "A tax subtype. Only required for dedicated cases. For vouchers referencing a B2C customer in the EU, and with a `taxType` of `net` or `gross`, the taxSubType may be set to **distanceSales**, or **electronicServices**. Passing a null value results in a standard voucher.  \nIf the organization's `distanceSalesPrinciple` ([profile endpoint](https://developers.lexoffice.io/docs/#profile-endpoint)) is set to `DESTINATION` *and* this attribute is set to **distanceSales** or **electronicServices**, the voucher needs to reference the destination country's tax rates."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_sub_type: Option<TaxSubType>,
    #[doc = "When *taxType* is set to a vat-free tax type then a note regarding the conditions can be set. When omitted lexoffice sets the organization's default."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_type_note: Option<String>,
}
#[doc = "The payment conditions are optional and the organization's or contact-specific defaults will be used if ommitted."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentConditions {
    #[doc = "A textual note regarding the payment conditions."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_label: Option<String>,
    #[doc = "A textual note regarding the payment conditions. This label template may contain variables such as the discount range. These variables are enclosed in curly braces, e.g., *{discountRange}*.'  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub payment_term_label_template: crate::marker::ReadOnly<String>,
    #[doc = "The time left (in days) until the payment must be conducted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_duration: Option<i64>,
    #[doc = "The payment discount conditions for the recurring-template."]
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
pub struct RecurringTemplateSettings {
    #[doc = "The id of the recurring template settings.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "(Optional) The start date of the first recurring invoice in short iso date `yyyy-MM-dd`. If null, recurring template is **PAUSED**."]
    #[serde(with = "crate::serde::optional_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub start_date: Option<crate::types::Date>,
    #[doc = "(Optional) The end date of the first recurring invoice in short iso date `yyyy-MM-dd`."]
    #[serde(with = "crate::serde::optional_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub end_date: Option<crate::types::Date>,
    #[doc = "Specifies the status of the invoice. If false recurring invoices are created as **draft** (is editable), otherwise they are finalized as **open** (finalized and no longer editable but yet unpaid or only partially paid). In contrast to the invoice endpoint, finalized recurring invoices will immediately and automatically be sent to the customer via email."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub finalize: Option<bool>,
    #[doc = "The same shipping types defined in the shipping conditions of invoices. Can be either one of: **service**, **serviceperiod**, **delivery**, **deliveryperiod**, **none**. The shipping dates/date range will be calculated automatically during execution."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping_type: Option<ShippingType>,
    #[doc = "The execution interval defined as **WEEKLY**, **BIWEEKLY**, **MONTHLY**, **QUARTERLY**, **BIANNUALLY**, **ANNUALLY**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub execution_interval: Option<ExecutionInterval>,
    #[doc = "Whether the last execution of the recurring template was successful or not.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub last_execution_failed: crate::marker::ReadOnly<bool>,
    #[doc = "Describes the problem briefly when the last execution has failed.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub last_execution_error_message: crate::marker::ReadOnly<String>,
    #[doc = "The status of the recurring template defined as **ACTIVE**, **PAUSED**, **ENDED**. Note, that there is no error state.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub execution_status: crate::marker::ReadOnly<ExecutionStatus>,
}
impl crate::request::HasId for RecurringTemplateSettings {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
