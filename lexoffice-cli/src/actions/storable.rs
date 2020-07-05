use edit::edit_with_builder as edit;
use lexoffice::request::Request;
use lexoffice::request::{Endpoint, ResultInfo, Storable};
use lexoffice::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_any::{from_str, to_string_pretty, Format};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct StorableOpt {}

impl StorableOpt {
    pub async fn exec<T, U>(
        &self,
        request: Request<T, U>,
        obj: T,
    ) -> Result<ResultInfo<T>>
    where
        Request<T, U>: Storable + Endpoint + Clone,
        T: Serialize + DeserializeOwned + Send + Clone,
        U: Clone,
    {
        let mut builder = edit::Builder::new();
        let new_str = edit(
            to_string_pretty(&obj, Format::Yaml).unwrap(),
            builder.suffix(".yaml"),
        )?;
        let new_obj: T = from_str(&new_str, Format::Yaml).unwrap();

        request.save(new_obj).await
    }
}
