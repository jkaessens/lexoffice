use lexoffice::request::Request;
use lexoffice::request::{ ById, Endpoint };
use structopt::StructOpt;
use lexoffice::Result;
use serde::de::DeserializeOwned;
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, StructOpt)]
pub struct ByIdOpt {
    id: String,
}

impl ByIdOpt {
    pub async fn exec<T>(self, request: Request<T, ()> ) -> Result<T>
        where Request<T, ()>: ById + Endpoint, T: DeserializeOwned
    {
        let id = Uuid::from_str(&self.id)?;
        request.by_id(id).await
    }
}
