use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::EventSubscription;
use lexoffice::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum EventSubscriptionOpt {
    List(PaginatedOpt),
    New(StorableOpt),
    Updatable(UpdatableOpt),
    Get(ByIdOpt),
    Delete(DeletableOpt),
}

fn default() -> EventSubscription {
    EventSubscription::builder()
        .event_type("contact.created".to_string())
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
