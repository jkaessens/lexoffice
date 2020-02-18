mod contacts;
mod credit_notes;
mod event_subscriptions;
mod files;
mod invoices;
mod order_confirmations;
mod profile;
mod quotations;
pub mod voucher_list;

mod paginated;

pub use self::paginated::PageStream;
pub use self::paginated::Paginated;
use crate::client::Client;
use crate::error::Error;
use crate::model::server_resource::ServerResource;
use crate::reqwest_ext::RequestBuilderExt;
use crate::result::Result;
use async_trait::async_trait;
use mime::APPLICATION_JSON;
use reqwest::header::CONTENT_TYPE;
use reqwest::Method;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Request<T> {
    client: Client,
    url: Url,
    target: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(client: Client) -> Self {
        let url = client.base_url().clone();
        Request {
            client,
            url,
            target: PhantomData,
        }
    }
}

impl<T> RequestTrait for Request<T>
where
    Self: Endpoint,
{
    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut().unwrap().push(Self::ENDPOINT);
        url
    }
    fn client(&self) -> &Client {
        &self.client
    }
}

pub trait Endpoint {
    const ENDPOINT: &'static str;
}

pub trait RequestTrait
where
    Self: Sized,
{
    fn url(&self) -> Url;
    fn client(&self) -> &Client;
}

#[async_trait]
pub trait Storable<T>
where
    Self: RequestTrait + Sync,
    T: Send + Serialize,
{
    async fn upload<I>(self, object: I) -> Result<ServerResource<T>>
    where
        I: Into<T> + Send + 'async_trait,
        T: 'async_trait,
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

#[async_trait]
pub trait Updatable<T>
where
    Self: RequestTrait + Sync,
    T: Send + Sync + Serialize,
{
    async fn upload<I>(self, object: I) -> Result<ServerResource<T>>
    where
        I: Into<ServerResource<T>> + Send + 'async_trait,
        T: 'async_trait,
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

#[async_trait]
pub trait ById<T>
where
    T: DeserializeOwned,
    Self: Sized + RequestTrait + Sync,
{
    fn by_id_url<I>(&self, uuid: I) -> Result<Url>
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

    async fn by_id_str(self, uuid: &str) -> Result<ServerResource<T>>
    where
        T: 'async_trait,
    {
        self.by_id(Uuid::from_str(uuid)?).await
    }

    async fn by_id<I>(self, uuid: I) -> Result<ServerResource<T>>
    where
        T: 'async_trait,
        I: Into<Uuid> + Send,
    {
        let url = self.by_id_url(uuid)?;
        self.client()
            .http_builder(Method::GET, url)
            .to_json_response()
            .await
    }
}
