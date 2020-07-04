use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use crate::Error;
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use uuid::Uuid;

/// This trait marks a `Request` as `ById`-requestable and unlocks the
/// `Request::by_id_url()`, `Request::by_id_str()`, `Request::by_id()` methods.
pub trait ById {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + ById,
    T: DeserializeOwned,
{
    /// This method creates an `Url` that is used to address the object
    /// identified by `uuid`.
    /// `Request<T>` must implement the `ById` trait in order to make
    /// this function available.
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

    /// This method requests an object identified by `uuid`.
    /// `Request<T>` must implement the `ById` trait in order to make
    /// this function available.
    pub async fn by_id_str(self, uuid: &str) -> Result<T> {
        self.by_id(Uuid::from_str(uuid)?).await
    }

    /// This method requests an object identified by `uuid`.
    /// `Request<T>` must implement the `ById` trait in order to make
    /// this function available.
    pub async fn by_id<I>(self, uuid: I) -> Result<T>
    where
        I: Into<Uuid> + Send,
    {
        let url = self.by_id_url(uuid)?;
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
