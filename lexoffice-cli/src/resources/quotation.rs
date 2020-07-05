use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::Quotation;
use lexoffice::Result;
use structopt::StructOpt;

/// quotation endpoint
#[derive(Debug, StructOpt)]
pub enum QuotationOpt {
    /// retrieves a paginated list of all quotations
    List(PaginatedOpt),
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    /// queries a specific quotation by its id
    Get(ByIdOpt),
}

impl QuotationOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Quotation>> {
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
