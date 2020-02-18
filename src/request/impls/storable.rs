use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::StateRequest;
use crate::reqwest_ext::RequestBuilderExt;
use crate::result::Result;
use reqwest::Method;
use std::marker::PhantomData;

pub trait Storable {}

impl<T, S> StateRequest<T, S>
where
    Self: Endpoint + Storable,
{
    pub async fn save<I>(self, object: I) -> Result<ServerResource<T>>
    where
        I: Into<T> + Send,
    {
        let object = object.into();
        let url = self.url();
        Ok(self
            .client()
            .http_builder(Method::POST, url)
            .to_json_response::<ServerResource<PhantomData<T>>>()
            .await?
            .wrap(object))
    }
}
