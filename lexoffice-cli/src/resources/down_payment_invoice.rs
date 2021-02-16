use crate::actions::*;
use crate::ReturnType;
use lexoffice::model::DownPaymentInvoice;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// invoice endpoint
#[derive(Debug, StructOpt)]
pub enum DownPaymentInvoiceOpt {
    /// queries a specific invoice by its id
    Get(ByIdOpt),
}

impl DownPaymentInvoiceOpt {
    pub async fn exec(
        &self,
        client: Client,
    ) -> Result<ReturnType<DownPaymentInvoice>> {
        let request = client.request::<DownPaymentInvoice>();
        let result = match self {
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
