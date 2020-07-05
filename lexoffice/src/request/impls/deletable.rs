use super::ById;
use crate::request::Endpoint;
use crate::request::HasId;
use crate::request::Request;
use crate::result::Result;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// This trait marks a `Request` as `Deletable` and unlocks the
/// `Request::delete()` method.
pub trait Deletable {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Deletable + ById,
    T: Serialize + DeserializeOwned + HasId + Clone,
    S: Clone,
{
    /// This method requests an object identified by `uuid`.
    /// `Request<T>` must implement the `ById` trait in order to make
    /// this function available.
    pub async fn delete_str(self, uuid: &str) -> Result<()> {
        self.delete(Uuid::from_str(uuid)?).await
    }

    /// This method requests an object identified by `uuid`.
    /// `Request<T>` must implement the `ById` trait in order to make
    /// this function available.
    pub async fn delete<I>(self, uuid: I) -> Result<()>
    where
        I: Into<Uuid> + Send,
    {
        let url = self.by_id_url(uuid)?;
        self.client()
            .http_builder(Method::DELETE, url)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
