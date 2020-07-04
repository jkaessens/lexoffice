use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::Invoice;
use lexoffice::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum InvoiceOpt {
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl InvoiceOpt {
    pub async fn exec(self, client: Client) -> Result<ReturnType<Invoice>> {
        let request = client.request::<Invoice>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
