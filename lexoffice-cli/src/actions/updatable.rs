use edit::edit;
use lexoffice::request::HasId;
use lexoffice::request::Request;
use lexoffice::request::{ById, Endpoint, ResultInfo, Updatable};
use lexoffice::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_any::{from_str, to_string_pretty, Format};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct UpdatableOpt {
    id: String,
}

impl UpdatableOpt {
    pub async fn exec<T, U>(
        &self,
        request: Request<T, U>,
    ) -> Result<ResultInfo<T>>
    where
        Request<T, U>: Updatable + ById + Endpoint + Clone,
        T: Serialize + DeserializeOwned + Send + HasId + Clone,
        U: Clone,
    {
        let get = request.clone();
        let obj: T = get.by_id_str(&self.id).await?;

        let new_str = edit(to_string_pretty(&obj, Format::Yaml).unwrap())?;
        let new_obj: T = from_str(&new_str, Format::Yaml).unwrap();

        request.update(new_obj).await
    }
}
