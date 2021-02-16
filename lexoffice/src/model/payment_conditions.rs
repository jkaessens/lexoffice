#![doc = "The payment conditions endpoint provides read access to the list of payment conditions configured in lexoffice."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[doc = "The payment conditions are optional and the organization's or contact-specific defaults will be used if ommitted."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PaymentCondition {
    #[doc = "The payment conditions' identifier"]
    #[builder(default, setter(skip))]
    pub id: crate::marker::ReadOnly<uuid::Uuid>,
    #[doc = "True for one payment condition object that was selected by the user as her default, false for all others"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_default: Option<bool>,
    #[doc = "A textual note regarding the payment conditions. This label template may contain variables such as the discount range. These variables are enclosed in curly braces, e.g., *{discountRange}*.'  \n*Read-only.*"]
    #[builder(default, setter(skip))]
    pub payment_term_label_template: crate::marker::ReadOnly<String>,
    #[doc = "The time left (in days) until the payment must be conducted."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_term_duration: Option<i64>,
    #[doc = "The payment discount conditions for the payment condition."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub payment_discount_conditions: Option<Vec<PaymentDiscountConditions>>,
}
impl crate::request::HasId for PaymentCondition {
    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
        &self.id
    }
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
