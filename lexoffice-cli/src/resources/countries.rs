use crate::ReturnType;
use lexoffice::model::Country;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// retrieves the users profile
#[derive(Debug, StructOpt)]
pub struct CountryOpt {}

impl CountryOpt {
    pub async fn exec(
        &self,
        client: Client,
    ) -> Result<ReturnType<Vec<Country>>> {
        let request = client.request::<Country>();

        Ok(ReturnType::Obj(request.get().await?))
    }
}
