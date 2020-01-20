use crate::client::Client;
use async_trait::async_trait;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub trait Resource {
    const BASE_PATH: &'static str;
}

#[async_trait]
pub trait SimpleResource: Resource {
    async fn save(self, client: Client) -> Result<Self, Box<dyn Error>>
    where
        Self: WriteableResouce
            + Serialize
            + DeserializeOwned
            + Sized
            + Send
            + Sync,
    {
        Ok(client
            .request(Method::POST, &Self::BASE_PATH)
            .json(&self)
            .send()
            .await?
            .json()
            .await?)
    }
}

pub trait WriteableResouce: Resource {}

pub trait PaginatedResource: Resource {}

pub trait ItemsResource: Resource {}
