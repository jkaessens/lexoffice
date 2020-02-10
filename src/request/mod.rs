mod contacts;
mod credit_notes;
mod event_subscriptions;
mod invoices;
mod order_confirmations;
mod profile;
pub mod voucher_list;
mod quotations;
mod paginated;

use crate::client::RequestBuilder;
use crate::model::server_resource::ServerResource;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::str::FromStr;
use std::sync::Arc;
use reqwest::Url;
use uuid::Uuid;
use crate::Result;
pub use self::paginated::Paginated;
pub use self::paginated::PageStream;

#[derive(Clone, Debug)]
pub struct Request<T> {
    builder: Arc<RequestBuilder>,
    url: Url,
    phantom: PhantomData<T>,
}

impl<T> Request<T> {
    pub fn new(builder: Arc<RequestBuilder>) -> Self {
        let url = builder.base_url.clone();
        Request {
            builder,
            url,
            phantom: PhantomData,
        }
    }
}

impl<T> Requestable for Request<T>
where
    Self: Endpoint,
{
    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut().unwrap().push(Self::ENDPOINT);
        url
    }
    fn builder(&self) -> &RequestBuilder {
        &self.builder
    }
}

pub trait Endpoint {
    const ENDPOINT: &'static str;
}

pub trait Requestable
where
    Self: Sized,
{
    fn url(&self) -> Url;
    fn builder(&self) -> &RequestBuilder;
}

#[async_trait]
pub trait Storable<T>
where
    Self: Requestable,
{
    async fn save(self) -> Result<ServerResource<T>>;
}

#[async_trait]
pub trait Simple<T>
where
    Self: Requestable,
    T: DeserializeOwned,
{
    async fn get(self) -> Result<ServerResource<T>>
    where
        T: 'async_trait,
    {
        let url = self.url();
        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }
}

#[async_trait]
pub trait ById<T>
where
    T: DeserializeOwned,
    Self: Sized + Requestable,
{
    fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| "cannot be base")?
            .push(&uuid.to_string());
        Ok(url)
    }

    async fn by_id_str(
        self,
        uuid: &str,
    ) -> Result<ServerResource<T>>
    where
        T: 'async_trait,
    {
        let builder = self.builder();
        let url = self.by_id_url(Uuid::from_str(uuid)?)?;
        Ok(builder.json(&url).await?)
    }
    async fn by_id<I>(
        self,
        uuid: I,
    ) -> Result<T>
    where
        T: 'async_trait,
        I: Into<Uuid> + Send + Sync,
    {
        let builder = self.builder();
        let url = self.by_id_url(uuid)?;
        Ok(builder.json(&url).await?)
    }
}
