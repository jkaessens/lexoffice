#![doc = "This endpoint provides read access to contacts (e.g. customers, vendors). A contact can hold addresses, contact information (e.g. phone numbers, email addresses) and contact persons for company related contacts. It is also possible to use filters on the contacts collection."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Contact {
    #[doc = "Unique id of the contact generated on creation by lexoffice."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the contact belongs to."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_id: Option<uuid::Uuid>,
    #[doc = "Version *(revision)* number which will be increased on each change to handle [optimistic locking](https://developers.lexoffice.io/docs/#optimistic-locking).  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub version: i64,
    #[doc = "Defines contact roles and supports further contact information. For object details see below."]
    #[builder(setter(into))]
    pub roles: Roles,
    #[doc = "Company related information. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub company: Option<Company>,
    #[doc = "Individual person related information. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub person: Option<Person>,
    #[doc = "Addresses (e.g. billing and shipping address(es)) for the contact. Contains a list for each address type. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub addresses: Option<Addresses>,
    #[doc = "XRechnung related properties of the contact"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub x_rechnung: Option<XRechnung>,
    #[doc = "Email addresses for the contact. Contains a list for each EMail type. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_addresses: Option<EmailAddresses>,
    #[doc = "Phone numbers for the contact. Contains a list for each PhoneNumber type. For details see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub phone_numbers: Option<PhoneNumbers>,
    #[doc = "A note to the contact. This is just an additional information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub note: Option<String>,
    #[doc = "Archived flag of the contact.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub archived: crate::marker::ReadOnly<bool>,
}
impl crate::request::HasId for Contact {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
#[doc = "Contains a customer and/or a vendor object. The presence of a role in the JSON implies that the contact will have this role. For example, if the customer object is present, the contact has the role customer.\nPlease note that each contact must have at least one role."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Roles {
    #[doc = "May be present. If present the created contact has the role customer."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub customer: Option<Customer>,
    #[doc = "May be present. If present the created contact has the role vendor."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vendor: Option<Vendor>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Customer {
    #[doc = "Unique customer number within the current organization. This number is created by lexoffice for contacts with role Customer. It cannot be set during creation and cannot be changed.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub number: crate::marker::ReadOnly<i64>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Vendor {
    #[doc = "Unique vendor number within the current organization. This number is created by lexoffice for contacts with role Vendor. It cannot be set during creation and cannot be changed.  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub number: crate::marker::ReadOnly<i64>,
}
#[doc = "Use this object to provide information for a contact of type company."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Company {
    #[doc = "Possible values are **true** or **false**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub allow_tax_free_invoices: Option<bool>,
    #[doc = "Company name"]
    #[builder(setter(into))]
    pub name: String,
    #[doc = "Tax number for this company --> *\"Steuernummer\"*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub tax_number: Option<String>,
    #[doc = "Vat registration id for this company. This id has to follow the german rules for the vat registration ids --> *\"Umsatzsteuer ID\"*."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vat_registration_id: Option<String>,
    #[doc = "A list of company contact persons. Each entry is an object of [company contact person](https://developers.lexoffice.io/docs/#company-contact-person-details). Details of nested object please see below."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_persons: Option<Vec<CompanyContactPerson>>,
}
#[doc = "Please note that it's only possible to create and change contacts with a maximum of one contact person. It's possible to retrieve contacts with more than one contact person, but it's not possible to update such a contact via the REST API."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CompanyContactPerson {
    #[doc = "Salutation for the contact person with max length of 25 characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub salutation: Option<String>,
    #[doc = "First name of the contact person."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub first_name: Option<String>,
    #[doc = "Last name of the contact person."]
    #[builder(setter(into))]
    pub last_name: String,
    #[doc = "Flags if contact person is the primary contact person. Primary contact persons are shown on vouchers. Default is **false**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub primary: Option<bool>,
    #[doc = "Email address of the contact person."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_address: Option<String>,
    #[doc = "Phone number of the contact person."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub phone_number: Option<String>,
}
#[doc = "Use this object to provide information for a contact of type private person."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Person {
    #[doc = "Salutation for the individual person with max length of 25 characters."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub salutation: Option<String>,
    #[doc = "First name of the person."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub first_name: Option<String>,
    #[doc = "Last name of the person."]
    #[builder(setter(into))]
    pub last_name: String,
}
#[doc = "Use this objects to provide billing and shipping information of a contact.\n\nPlease note that it's only possible to create and change contacts with a maximum of one billing and/or one shipping address. It's possible to retrieve contacts with more than one billing and shipping address, but it's not possible to update such a contact via the REST API."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Addresses {
    #[doc = "A list of billing addresses. Each entry is an object of [address](https://developers.lexoffice.io/docs/#address-details)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub billing: Option<Vec<Address>>,
    #[doc = "A list of shipping addresses. Each entry is an object of [address](https://developers.lexoffice.io/docs/#address-details)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub shipping: Option<Vec<Address>>,
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Address {
    #[doc = "Additional address information."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub supplement: Option<String>,
    #[doc = "Street with Street number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub street: Option<String>,
    #[doc = "Zip code"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub zip: Option<String>,
    #[doc = "City"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub city: Option<String>,
    #[doc = "Country code in the format of [ISO 3166 alpha2](https://developers.lexoffice.io/docs/#faq-country-codes) (e.g. DE is used for germany)."]
    #[builder(setter(into))]
    pub country_code: crate::types::CountryCode,
}
#[doc = "Contacts for German public authorities should be created with both of the following attributes set.\nThis results in the generation of invoice documents conforming to the German XRechnung standard when creating invoices in lexoffice.\n\nIf a customer's `buyerReference` is set, its `vendorNumberAtCustomer` needs to be set as well."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct XRechnung {
    #[doc = "Customer's *Leitweg-ID* conforming to the German XRechnung system"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub buyer_reference: Option<String>,
    #[doc = "Your vendor number as used by the customer"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub vendor_number_at_customer: Option<String>,
}
#[doc = "Please note that it's only possible to create and change contacts with a maximum of one entry in each of the below described lists. It's possible to retrieve contacts with more than one entry in the lists, but it's not possible to update such a contact via the REST API."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EmailAddresses {
    #[doc = "A list of email addresses. Each entry is of type string and contains an email address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub business: Option<Vec<String>>,
    #[doc = "A list of email addresses. Each entry is of type string and contains an email address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub office: Option<Vec<String>>,
    #[doc = "A list of email addresses. Each entry is of type string and contains an email address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<Vec<String>>,
    #[doc = "A list of email addresses. Each entry is of type string and contains an email address."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub other: Option<Vec<String>>,
}
#[doc = "Please note that it's only possible to create and change contacts with a maximum of one entry in each of the below described lists. It's possible to retrieve contacts with more than one entry in the lists, but it's not possible to update such a contact via the REST API."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PhoneNumbers {
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub business: Option<Vec<String>>,
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub office: Option<Vec<String>>,
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub mobile: Option<Vec<String>>,
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub private: Option<Vec<String>>,
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub fax: Option<Vec<String>>,
    #[doc = "A list of phone numbers. Each entry is of type string and contains a phone number."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub other: Option<Vec<String>>,
}
