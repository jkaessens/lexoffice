use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::Voucher;
use lexoffice::Result;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum VoucherOpt {
    //New(StorableOpt),
    //Updatable(UpdatableOpt),
    Get(ByIdOpt),
}

impl VoucherOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Voucher>> {
        let request = client.request::<Voucher>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
