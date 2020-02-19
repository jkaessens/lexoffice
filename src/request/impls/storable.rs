use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use reqwest::Method;
use std::marker::PhantomData;

/// This trait marks a `Request` as `Updatable` and unlocks the
/// `Request::update` method.
pub trait Storable {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Storable,
{
    /// This method allows to save a new model object. Please note, that
    /// `Request<T>` must implement the `Storable` trait in order to make
    /// this function available.
    pub async fn save<I>(self, object: I) -> Result<ServerResource<T>>
    where
        I: Into<T> + Send,
    {
        let object = object.into();
        let url = self.url();
        Ok(to_json_response::<ServerResource<PhantomData<T>>>(
            self.client().http_builder(Method::POST, url),
        )
        .await?
        .wrap(object))
    }
}
