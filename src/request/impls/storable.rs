use crate::model::server_resource::ServerResource;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use reqwest::Method;
use std::marker::PhantomData;

pub trait Storable {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Storable,
{
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
