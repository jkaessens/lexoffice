# ! [ doc = "Using event subscriptions you will be notified about certain events on resources \\- e.g. you receive a notification every time a contact changes in lexoffice. This will make pull requests superfluous to keep your data synced between lexoffice and your application. The notifications are implemented as webhooks. Subscribing to an event simply requires the *event type* and your *callback url*. With the event\\-subscriptions endpoint you can manage your subscriptions within lexoffice." ]use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
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
    pub created_date:
        super::super::marker::ReadOnly<chrono::DateTime<chrono::Utc>>,
    #[doc = "The event type is a combined key which defines the resource and its event name you are subscribing to. All available events receivable via the API can be taken from the table [Event Types](#event-subscriptions-endpoint-event-types)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub event_type: Option<String>,
    #[doc = "When a resource entity triggers an event, the callback url is used to notify the subscriber about it. The payload of the callback is described in [Webhook Callback Properties](#event-subscriptions-endpoint-webhook-callback-properties)."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub callback_url: Option<String>,
}
impl super::super::request::HasId for EventSubscription {
    fn id(&self) -> &super::super::marker::ReadOnly<uuid::Uuid> {
        &self.subscription_id
    }
}
