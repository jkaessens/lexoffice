use crate::actions::*;
use crate::ReturnType;
use lexoffice::model::Payment;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// invoice endpoint
#[derive(Debug, StructOpt)]
pub enum PaymentOpt {
    /// queries a specific invoice by its id
    Get(ByIdOpt),
}

impl PaymentOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Payment>> {
        let request = client.request::<Payment>();
        let result = match self {
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
