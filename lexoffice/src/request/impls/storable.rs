use crate::request::Endpoint;
use crate::request::Request;
use crate::request::ResultInfo;
use crate::result::Result;
use crate::util::to_json_response;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// This trait marks a `Request` as `Storable` and unlocks the
/// `Request::save()` method.
pub trait Storable {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Storable,
    T: DeserializeOwned + Serialize + Clone,
    S: Clone,
{
    /// This method allows to save a new model object. Please note, that
    /// `Request<T>` must implement the `Storable` trait in order to make
    /// this function available.
    pub async fn save<I>(self, object: I) -> Result<ResultInfo<T>>
    where
        I: Into<T> + Send,
    {
        let object = object.into();
        let url = self.url();
        Ok(to_json_response::<ResultInfo<T>>(
            self.client().http_builder(Method::POST, url).json(&object),
        )
        .await?)
    }
}
