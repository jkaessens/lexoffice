use edit::edit_with_builder as edit;
use lexoffice::request::HasId;
use lexoffice::request::Request;
use lexoffice::request::{ById, Endpoint, ResultInfo, Updatable};
use lexoffice::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_any::{from_str, to_string_pretty, Format};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct UpdatableOpt {
    /// uuid of the element
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
        let object: T = get.by_id_str(&self.id).await?;
        let id = object.id().ok_or(Error::NoUuid)?;

        let new_str = edit(
            to_string_pretty(&object, Format::Yaml).unwrap(),
            edit::Builder::new().suffix(".yaml"),
        )?;
        let new_obj: T = from_str(&new_str, Format::Yaml).unwrap();

        request.update_with_id(id, new_obj).await
    }
}
