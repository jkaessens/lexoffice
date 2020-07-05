use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::CreditNote;
use lexoffice::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum CreditNoteOpt {
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl CreditNoteOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<CreditNote>> {
        let request = client.request::<CreditNote>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
