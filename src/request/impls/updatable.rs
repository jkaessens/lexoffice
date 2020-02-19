use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use serde::Serialize;
use std::marker::PhantomData;

pub trait Updatable {}

impl<T, S> Request<T, S>
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
        Ok(to_json_response::<ServerResource<PhantomData<T>>>(
            self.client()
                .http_builder(Method::PUT, url)
                .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
                .json(&object),
        )
        .await?
        .wrap(object.inner))
    }
}
