use crate::actions::*;
use crate::ReturnType;
use lexoffice::model::OrderConfirmation;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// order confirmation endpoint
#[derive(Debug, StructOpt)]
pub enum OrderConfirmationOpt {
    /// queries a specific order confirmation by its id
    Get(ByIdOpt),
}

impl OrderConfirmationOpt {
    pub async fn exec(
        &self,
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
