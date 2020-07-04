use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::OrderConfirmation;
use lexoffice::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum OrderConfirmationOpt {
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl OrderConfirmationOpt {
    pub async fn exec(
        self,
        client: Client,
    ) -> Result<ReturnType<OrderConfirmation>> {
        let request = client.request::<OrderConfirmation>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
