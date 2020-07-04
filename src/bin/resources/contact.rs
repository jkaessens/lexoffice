use structopt::StructOpt;
use crate::actions::*;
use lexoffice::client::Client;
use lexoffice::model::Contact;
use lexoffice::Result;
use serde::Serialize;
use crate::ReturnType;

#[derive(Debug, StructOpt)]
pub enum ContactOpt {
    List(PaginatedOpt),
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl ContactOpt {
    pub async fn exec(self, client: Client) -> Result<ReturnType<Contact>> {
        let request = client.request::<Contact>();
        let result = match self {
            Self::List(x) => ReturnType::Paged(x.exec(request).await?),
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
