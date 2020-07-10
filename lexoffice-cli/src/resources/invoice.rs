use crate::actions::*;
use crate::ReturnType;
use lexoffice::Client;
use lexoffice::model::Invoice;
use lexoffice::Result;
use structopt::StructOpt;

/// invoice endpoint
#[derive(Debug, StructOpt)]
pub enum InvoiceOpt {
    /// queries a specific invoice by its id
    Get(ByIdOpt),
}

impl InvoiceOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Invoice>> {
        let request = client.request::<Invoice>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
