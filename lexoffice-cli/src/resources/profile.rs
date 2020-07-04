use structopt::StructOpt;
use crate::actions::*;
use lexoffice::client::Client;
use lexoffice::model::Profile;
use lexoffice::Result;
use serde::Serialize;
use crate::ReturnType;

#[derive(Debug, StructOpt)]
pub struct ProfileOpt {
}

impl ProfileOpt {
    pub async fn exec(self, client: Client) -> Result<ReturnType<Profile>> {
        let request = client.request::<Profile>();

        Ok(ReturnType::Obj(request.get().await?))
    }
}
