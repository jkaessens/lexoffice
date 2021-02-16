use crate::actions::*;
use crate::ReturnType;
use lexoffice::model::event_subscriptions::EventType;
use lexoffice::model::EventSubscription;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// event subscription endpoint
#[derive(Debug, StructOpt)]
pub enum EventSubscriptionOpt {
    /// retrieves a paginated list of all event subscription
    List(PaginatedOpt),
    /// creates new event subscription and opens it in an editor
    New(StorableOpt),
    /// modifies an event subscription
    Updatable(UpdatableOpt),
    /// queries a specific event subscription by its id
    Get(ByIdOpt),
    /// deletes a event subscription
    Delete(DeletableOpt),
}

fn default() -> EventSubscription {
    EventSubscription::builder()
        .event_type(EventType::ContactCreated)
        .callback_url("http://example.com/callback".to_string())
        .build()
}

impl EventSubscriptionOpt {
    pub async fn exec(
        &self,
        client: Client,
    ) -> Result<ReturnType<EventSubscription>> {
        let request = client.request::<EventSubscription>();
        let result = match self {
            Self::List(x) => ReturnType::Paged(x.exec(request).await?),
            Self::New(x) => {
                ReturnType::ResultInfo(x.exec(request, default()).await?)
            }
            Self::Updatable(x) => {
                ReturnType::ResultInfo(x.exec(request).await?)
            }
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
            Self::Delete(x) => {
                x.exec(request).await?;
                ReturnType::Empty
            }
        };
        Ok(result)
    }
}
