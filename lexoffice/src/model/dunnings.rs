#![doc = "This endpoint provides read and write access to dunnings and also the possibility to render the document as a PDF in order to download it. Dunnings are always created in draft mode and do not need to be finalized.\n\nA dunning requires an invoice as a reference, making the `precedingSalesVoucherId` a mandatory query parameter. When creating a dunning, the contact ids of the invoice and the dunning must be equal (or both be absent, resulting in a reference to the collective customer). The name attribute in the address field is copied from the referenced invoice and will be ignored in the dunning structure. The tax conditions must match the tax conditions in the referenced invoice.\n\nDunning a down payment invoice is possible as well."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VoucherStatus {
    #[serde(rename = "draft")]
    Draft,
}
impl std::str::FromStr for VoucherStatus {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[doc = "```json\n{\n   \"id\":\"e9066f04-8cc7-4616-93f8-ac9ecc8479c8\",\n   \"organizationId\":\"aa93e8a8-2aa3-470b-b914-caad8a255dd8\",\n   \"createdDate\":\"2021-07-17T18:32:07.480+02:00\",\n   \"updatedDate\":\"2021-07-17T18:32:07.551+02:00\",\n   \"version\":1,\n   \"language\":\"de\",\n   \"archived\":false,\n   \"voucherStatus\":\"draft\",\n   \"voucherDate\":\"2021-07-17T00:00:00.000+01:00\",\n   \"address\":{\n      \"supplement\":\"Gebäude 10\",\n      \"street\":\"Musterstraße 42\",\n      \"city\":\"Freiburg\",\n      \"zip\":\"79112\",\n      \"countryCode\":\"DE\"\n   },\n   \"lineItems\":[\n      {\n         \"type\": \"custom\",\n         \"name\": \"Energieriegel Testpaket\",\n         \"quantity\": 1,\n         \"unitName\": \"Stück\",\n         \"unitPrice\": {\n            \"currency\": \"EUR\",\n            \"netAmount\": 5,\n            \"grossAmount\": 5.0,\n            \"taxRatePercentage\": 0.0\n         },\n         \"discountPercentage\": 0,\n         \"lineItemAmount\": 5.0\n      },\n      {\n         \"type\": \"text\",\n         \"name\": \"Strukturieren Sie Ihre Belege durch Text-Elemente.\",\n         \"description\": \"Das hilft beim Verst\\u00e4ndnis\"\n      }\n   ],\n   \"totalPrice\": {\n       \"currency\": \"EUR\",\n       \"totalNetAmount\": 5.0,\n       \"totalGrossAmount\": 5.0,\n       \"totalTaxAmount\": 0.0\n   },\n   \"taxAmounts\": [\n       {\n           \"taxRatePercentage\": 0.0,\n           \"taxAmount\": 0.0,\n           \"netAmount\": 5.0\n       }\n   ],\n   \"taxConditions\": {\n       \"taxType\": \"net\"\n   },\n   \"shippingConditions\": {\n       \"shippingDate\": \"2021-07-21T15:16:44.051+02:00\",\n       \"shippingType\": \"delivery\"\n   },\n   \"relatedVouchers\": [\n       {\n           \"id\": \"52cd26a2-ea26-11eb-a4f0-2bb179f80c5a\",\n           \"voucherNumber\": \"RE0357\",\n           \"voucherType\": \"invoice\"\n       }\n   ],\n   \"introduction\": \"Wir bitten Sie, die nachfolgend aufgelisteten Lieferungen/Leistungen unverzüglich zu begleichen.\",\n   \"remark\": \"Sollten Sie den offenen Betrag bereits beglichen haben, betrachten Sie dieses Schreiben als gegenstandslos.\",\n   \"files\": {\n       \"documentFileId\": \"4e19354c-ea26-11eb-a31f-af2d58e85357\"\n   },\n   \"title\": \"Mahnung\"\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Dunning {
    #[doc = "Unique id generated on creation by lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the dunning belongs to.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the dunning was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The instant of time when the dunning was updated by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub updated_date: crate::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Specifies the language of the dunning which affects the print document but also set translated default text modules when no values are send (e.g. for introduction). Values accepted in ISO 639-1 code. Possible values are German **de** (default) and English **en**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub language: Option<String>,
    #[doc = "Specifies if the dunning is only available in the archive in lexoffice.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub archived: crate::marker::ReadOnly<bool>,
    #[doc = "Specifies the status of the order confirmation. The only possible status is **draft** (is editable).   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_status: crate::marker::ReadOnly<VoucherStatus>,
    #[doc = "The date of dunning in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020-02-21T00:00:00.000+01:00*)."]
    #[builder(setter(into))]
    pub voucher_date: crate::types::DateTime,
    #[doc = "The address of the dunning recipient. For details see below."]
    #[builder(setter(into))]
    pub address: Address,
    #[doc = "The items of the dunning. For details see below."]
    #[builder(setter(into))]
    pub line_items: Vec<LineItems>,
    #[doc = "The related vouchers of the invoice. *Read-only.*"]
    #[builder(default, setter(skip))]
    pub related_vouchers: crate::marker::ReadOnly<Vec<RelatedVouchers>>,
    #[doc = "(Optional) A title text. The organization's default is used if no value was sent."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub title: Option<String>,
    #[doc = "(Optional) An introductory text / header. The organization's default is used if no value was send. **We recommended to include the invoice number in the header when the dunning is related to an invoice.**"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub introduction: Option<String>,
    #[doc = "(Optional) A closing text note. The organization's default is used if no value was send."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub remark: Option<String>,
    #[doc = "The document id for the PDF version of the dunning. For details see below.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub files: crate::marker::ReadOnly<Files>,
}
impl crate::request::HasId for Dunning {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "There are two main options to address the recipient of a dunning. First, using an existing lexoffice contact or second, creating a new address.\n\nFor **referencing an existing contact** it is only necessary to provide the UUID of that contact. Usually the billing address is used (for delivery notes, the shipping address will be preferred). Additionally, the referenced address can also be modified for this specific dunning. This can be done by setting all required address fields and this deviated address will not be stored back to the lexoffice contacts.\n\nThe referenced contact needs to have the role customer. For more information please refer to the [contacts endpoint](https://developers.lexoffice.io/docs/#contacts-endpoint).\n\nOtherwise, a **new address** for the dunning recipient can be created. That type of address is called a \"one-time address\". A one-time address will not create a new contact in lexoffice. For instance, this could be useful when it is not needed to create a contact in lexoffice for each new dunning.\n\nPlease get in touch with us if you are not sure which option fits your use case best."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "If the dunning recipient is (optionally) registered as a contact in lexoffice, this field specifies the related id of the contact."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_id: Option<uuid::Uuid>,
    #[doc = "The name of the dunning recipient. To use an existing contact of an individual person, provide the name in the format {firstname} {lastname}."]
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
    #[doc = "The tax rate of the unit price. See [the \"Supported tax rates\" FAQ](https://developers.lexoffice.io/docs/#faq-valid-tax-rates) for more information and a list of possible values.. For vat-free sales vouchers the tax rate percentage must be **0**."]
    #[builder(setter(into))]
    pub tax_rate_percentage: f64,
}
#[doc = "```json\n\"taxConditions\": {\n    \"taxType\": \"constructionService13b\",\n    \"taxTypeNote\": \"Steuerschuldnerschaft des Leistungsempfängers (Reverse Charge)\"\n}\n\n```"]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TaxConditions {
    #[doc = "The tax type for the dunning. Possible values are **net**, **gross**, **vatfree** (*Steuerfrei*), **intraCommunitySupply** (*Innergemeinschaftliche Lieferung gem. §13b UStG*), **constructionService13b** (*Bauleistungen gem. §13b UStG*), **externalService13b** (*Fremdleistungen innerhalb der EU gem. §13b UStG*), **thirdPartyCountryService** (*Dienstleistungen an Drittländer*), and **thirdPartyCountryDelivery** (*Ausfuhrlieferungen an Drittländer*)"]
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
#[doc = "The *relatedVouchers* property documents all existing voucher relations for the current sales voucher. If no related vouchers exist, an empty list will be returned."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelatedVouchers {
    #[doc = "The related sales voucher's unique id."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The specific number of the related sales voucher.   \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub voucher_number: crate::marker::ReadOnly<String>,
    #[doc = "Voucher type of the related sales voucher."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub voucher_type: Option<String>,
}
impl crate::request::HasId for RelatedVouchers {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Files {
    #[doc = "The id of the order confirmation PDF. To download the order confirmation PDF file please use the files endpoint."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub document_file_id: Option<uuid::Uuid>,
}
