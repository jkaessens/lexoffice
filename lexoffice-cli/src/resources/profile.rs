use crate::ReturnType;
use lexoffice::model::Profile;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// retrieves the users profile
#[derive(Debug, StructOpt)]
pub struct ProfileOpt {}

impl ProfileOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Profile>> {
        let request = client.request::<Profile>();

        Ok(ReturnType::Obj(request.get().await?))
    }
}
