use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::Quotation;
use lexoffice::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum QuotationOpt {
    List(PaginatedOpt),
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl QuotationOpt {
    pub async fn exec(self, client: Client) -> Result<ReturnType<Quotation>> {
        let request = client.request::<Quotation>();
        let result = match self {
            Self::List(x) => ReturnType::Paged(x.exec(request).await?),
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
