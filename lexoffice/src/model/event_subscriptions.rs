#![doc = "Using event subscriptions you will be notified about certain events on resources \\- e.g. you receive a notification every time a contact changes in lexoffice. This will make pull requests superfluous to keep your data synced between lexoffice and your application. The notifications are implemented as webhooks. Subscribing to an event simply requires the *event type* and your *callback url*. With the event\\-subscriptions endpoint you can manage your subscriptions within lexoffice."]
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[doc = "The following table lists all types of events you can subscribe to. The property *EventType* is the combined key of a resource and a event name. The *EventType* is handled in lower case."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum EventType {
    #[doc = "A new contact was created in lexoffice."]
    #[serde(rename = "contact.created")]
    ContactCreated,
    #[doc = "A lexoffice contact has changed. You should get the updated contact details."]
    #[serde(rename = "contact.changed")]
    ContactChanged,
    #[doc = "A lexoffice contact was deleted. Depending on your application, you should unlink the lexoffice contact on your site or delete it as well."]
    #[serde(rename = "contact.deleted")]
    ContactDeleted,
    #[doc = "A new credit note was created in lexoffice. Get the new credit note by calling the [credit notes endpoint](https://developers.lexoffice.io/docs/#credit-notes-endpoint)."]
    #[serde(rename = "credit-note.created")]
    CreditNoteCreated,
    #[doc = "A credit note has changed. Get the updated credit note by calling the [credit notes endpoint](https://developers.lexoffice.io/docs/#credit-notes-endpoint). Please note that the status may also have changed."]
    #[serde(rename = "credit-note.changed")]
    CreditNoteChanged,
    #[doc = "A credit note was deleted in lexoffice."]
    #[serde(rename = "credit-note.deleted")]
    CreditNoteDeleted,
    #[doc = "The status of a credit note has changed. Update the credit note by calling the [credit notes endpoint](https://developers.lexoffice.io/docs/#credit-notes-endpoint) to retrieve the new status."]
    #[serde(rename = "credit-note.status.changed")]
    CreditNoteStatusChanged,
    #[doc = "A new invoice was created in lexoffice. Get the new invoice by calling the [invoices endpoint](https://developers.lexoffice.io/docs/#invoices-endpoint)."]
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[doc = "An invoice has changed. Get the updated invoice by calling the [invoices endpoint](https://developers.lexoffice.io/docs/#invoices-endpoint). Please note that the status may also have changed."]
    #[serde(rename = "invoice.changed")]
    InvoiceChanged,
    #[doc = "An invoice was deleted in lexoffice."]
    #[serde(rename = "invoice.deleted")]
    InvoiceDeleted,
    #[doc = "The status of an invoice has changed. Update the invoice by calling the [invoices endpoint](https://developers.lexoffice.io/docs/#invoices-endpoint) to retrieve the new status."]
    #[serde(rename = "invoice.status.changed")]
    InvoiceStatusChanged,
    #[doc = "A new order confirmation was created in lexoffice. Get the new order confirmation by calling the [order confirmations endpoint](https://developers.lexoffice.io/docs/#order-confirmations-endpoint)."]
    #[serde(rename = "order-confirmation.created")]
    OrderConfirmationCreated,
    #[doc = "An order confirmation has changed. Get the updated order confirmation by calling the [order confirmations endpoint](https://developers.lexoffice.io/docs/#order-confirmations-endpoint). Please note that the status may also have changed."]
    #[serde(rename = "order-confirmation.changed")]
    OrderConfirmationChanged,
    #[doc = "An order confirmation was deleted in lexoffice."]
    #[serde(rename = "order-confirmation.deleted")]
    OrderConfirmationDeleted,
    #[doc = "The status of an order confirmation has changed. Update the order confirmation by calling the [order confirmations endpoint](https://developers.lexoffice.io/docs/#order-confirmations-endpoint) to retrieve the new status."]
    #[serde(rename = "order-confirmation.status.changed")]
    OrderConfirmationStatusChanged,
    #[doc = "A new quotation was created in lexoffice. Get the new quotation by calling the [quotations endpoint](https://developers.lexoffice.io/docs/#quotations-endpoint)."]
    #[serde(rename = "quotation.created")]
    QuotationCreated,
    #[doc = "A quotation has changed. Get the updated quotation by calling the [quotations endpoint](https://developers.lexoffice.io/docs/#quotations-endpoint). Please note that the status may also have changed."]
    #[serde(rename = "quotation.changed")]
    QuotationChanged,
    #[doc = "A quotation was deleted in lexoffice."]
    #[serde(rename = "quotation.deleted")]
    QuotationDeleted,
    #[doc = "The status of a quotation has changed. Update the quotation by calling the [quotations endpoint](https://developers.lexoffice.io/docs/#quotations-endpoint) to retrieve the new status."]
    #[serde(rename = "quotation.status.changed")]
    QuotationStatusChanged,
    #[doc = "The refresh token was revoked, hence is invalid. The `resourceId` in the webhook callback refers to the `connectionId` you retrieve using the [profile endpoint](https://developers.lexoffice.io/docs/#profile-endpoint). Please store the refresh token to the `connectionId` prior to the registration on this event."]
    #[serde(rename = "token.revoked")]
    TokenRevoked,
    #[doc = "A new (bookkeeping) voucher was created in lexoffice. Get the new voucher by calling the [vouchers endpoint](https://developers.lexoffice.io/docs/#vouchers-endpoint)."]
    #[serde(rename = "voucher.created")]
    VoucherCreated,
    #[doc = "A voucher has changed. Get the updated voucher by calling the [vouchers endpoint](https://developers.lexoffice.io/docs/#vouchers-endpoint)."]
    #[serde(rename = "voucher.changed")]
    VoucherChanged,
    #[doc = "A voucher was deleted in lexoffice."]
    #[serde(rename = "voucher.deleted")]
    VoucherDeleted,
    #[doc = "The status of a voucher has changed. Get the updated voucher by calling the [vouchers endpoint](https://developers.lexoffice.io/docs/#vouchers-endpoint)."]
    #[serde(rename = "voucher.status.changed")]
    VoucherStatusChanged,
}
impl std::str::FromStr for EventType {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s)
    }
}
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventSubscription {
    #[doc = "Unique id of the event subscription generated on creation by lexoffice.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub subscription_id: super::super::marker::ReadOnly<uuid::Uuid>,
    #[doc = "Unique id of the organization the event subscription belongs to.   \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub organization_id: super::super::marker::ReadOnly<uuid::Uuid>,
    #[doc = "The instant of time when the event subscription was created by lexoffice in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub created_date: super::super::marker::ReadOnly<crate::types::DateTime>,
    #[doc = "The event type is a combined key which defines the resource and its event name you are subscribing to. All available events receivable via the API can be taken from the table [Event Types](https://developers.lexoffice.io/docs/#event-subscriptions-endpoint-event-types)."]
    #[builder(setter(into))]
    pub event_type: EventType,
    #[doc = "When a resource entity triggers an event, the callback url is used to notify the subscriber about it. The payload of the callback is described in [Webhook Callback Properties](https://developers.lexoffice.io/docs/#event-subscriptions-endpoint-webhook-callback-properties)."]
    #[builder(setter(into))]
    pub callback_url: String,
}
impl super::super::request::HasId for EventSubscription {
    fn id(&self) -> &super::super::marker::ReadOnly<uuid::Uuid> {
        &self.subscription_id
    }
}
#[doc = "Subscribed events will send a POST request to your given webhook url and contain the following JSON payload."]
#[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
#[builder(doc)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WebhookCallback {
    #[doc = "The organization for which an event has been triggered."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub organization_id: Option<uuid::Uuid>,
    #[doc = "Describes the occurred event. The eventType describes the resource and the event name."]
    #[builder(setter(into))]
    pub event_type: EventType,
    #[doc = "The resource entity on which the event has occurred. Use the corresponding resource endpoint and the resourceId to get the latest data of the resource entity."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub resource_id: Option<uuid::Uuid>,
    #[doc = "The instant of time when the event was triggered in format `yyyy-MM-ddTHH:mm:ss.SSSXXX` as described in RFC 3339/ISO 8601 (e.g. *2020\\-02\\-21T00:00:00.000\\+01:00*).  \n*Read\\-only.*"]
    #[builder(default, setter(skip))]
    pub event_date: super::super::marker::ReadOnly<crate::types::DateTime>,
}
