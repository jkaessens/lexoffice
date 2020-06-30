use crate::request::Endpoint;
use crate::request::HasId;
use crate::request::Request;
use crate::request::ResultInfo;
use crate::result::Result;
use crate::util::to_json_response;
use crate::Error;
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

/// This trait marks a `Request` as `Updatable` and unlocks the
/// `Request::update()` method.
pub trait Updatable {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Updatable,
    T: Serialize + DeserializeOwned + HasId,
{
    /// This method allows to update an existing model object. Please note, that
    /// `Request<T>` must implement the `Updatable` trait in order to make
    /// this function available.
    pub async fn update<I>(self, object: I) -> Result<ResultInfo<T>>
    where
        I: Into<T> + Send,
    {
        let object = object.into();
        let mut url = self.url().clone();
        let uuid = object.id().ok_or(Error::NoUuid)?;
        url.path_segments_mut().unwrap().push(&uuid.to_string());
        Ok(to_json_response::<ResultInfo<T>>(
            self.client()
                .http_builder(Method::PUT, url)
                .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
                .json(&object),
        )
        .await?)
    }
}
