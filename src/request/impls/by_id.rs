use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::StateRequest;
use crate::result::Result;
use crate::util::to_json_response;
use crate::Error;
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use uuid::Uuid;

pub trait ById {}

impl<T, S> StateRequest<T, S>
where
    Self: Endpoint + ById,
    T: DeserializeOwned,
{
    pub fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid>,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| Error::UrlCannotBeBase)?
            .push(&uuid.to_string());
        Ok(url)
    }

    pub async fn by_id_str(self, uuid: &str) -> Result<ServerResource<T>> {
        self.by_id(Uuid::from_str(uuid)?).await
    }

    pub async fn by_id<I>(self, uuid: I) -> Result<ServerResource<T>>
    where
        I: Into<Uuid> + Send,
    {
        let url = self.by_id_url(uuid)?;
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
