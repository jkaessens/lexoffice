use structopt::StructOpt;
use crate::actions::*;
use lexoffice::client::Client;
use lexoffice::model::EventSubscription;
use lexoffice::Result;
use serde::Serialize;
use crate::ReturnType;

#[derive(Debug, StructOpt)]
pub enum EventSubscriptionOpt {
    List(PaginatedOpt),
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl EventSubscriptionOpt {
    pub async fn exec(self, client: Client) -> Result<ReturnType<EventSubscription>> {
        let request = client.request::<EventSubscription>();
        let result = match self {
            Self::List(x) => ReturnType::Paged(x.exec(request).await?),
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
