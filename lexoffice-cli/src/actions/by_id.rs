use lexoffice::request::Request;
use lexoffice::request::{ById, Endpoint};
use lexoffice::Result;
use serde::de::DeserializeOwned;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ByIdOpt {
    id: String,
}

impl ByIdOpt {
    pub async fn exec<T, U>(&self, request: Request<T, U>) -> Result<T>
    where
        Request<T, U>: ById + Endpoint + Clone,
        T: DeserializeOwned + Clone,
        U: Clone,
    {
        request.by_id_str(&self.id).await
    }
}
