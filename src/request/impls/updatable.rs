use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::StateRequest;
use crate::reqwest_ext::RequestBuilderExt;
use crate::result::Result;
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use serde::Serialize;
use std::marker::PhantomData;

pub trait Updatable {}

impl<T, S> StateRequest<T, S>
where
    Self: Endpoint + Updatable,
    T: Serialize,
{
    pub async fn update<I>(self, object: I) -> Result<ServerResource<T>>
    where
        I: Into<ServerResource<T>> + Send,
    {
        let object = object.into();
        let url = self.url();
        Ok(self
            .client()
            .http_builder(Method::PUT, url)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .json(&object)
            .to_json_response::<ServerResource<PhantomData<T>>>()
            .await?
            .wrap(object.inner))
    }
}
