#![doc = "This endpoint provides read access to the list of posting categories for the (bookkeeping) vouchers revenue or expense which are supported in lexoffice.\n\nCategory ids with type *income* can be used for revenue vouchers such as *salesinvoice* and *salescreditnote* and posting categories with type *outgo* can be applied for expense vouchers with voucher types *purchaseinvoice* or *purchasecreditnote*."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PostingCategory {
    #[doc = "Unique id of the posting category."]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Name of the posting category."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[doc = "Type of the posting category. Possible values are **income** for revenues and **outgo** for expenses."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub _type: Option<String>,
    #[doc = "Flags if a referenced contact is required for this posting category. Possible values are **true** and **false**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub contact_required: Option<bool>,
    #[doc = "Flags if items with different tax rate percentages (e.g. 7% and 19%) are allowed for this posting category. Possible values are **true** and **false**."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub split_allowed: Option<bool>,
    #[doc = "Name of the top level posting category."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub group_name: Option<String>,
}
impl crate::request::HasId for PostingCategory {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
}
