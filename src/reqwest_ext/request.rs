use crate::response::ResponseExt;
use crate::Result;
use async_trait::async_trait;
use mime::APPLICATION_JSON;
use reqwest::header::ACCEPT;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait RequestBuilderExt {
    async fn to_json_response<T>(self) -> Result<T>
    where
        T: DeserializeOwned;
}

#[async_trait]
impl RequestBuilderExt for RequestBuilder {
    async fn to_json_response<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .header(ACCEPT, APPLICATION_JSON.as_ref())
            .send()
            .await?
            .error_for_lexoffice()
            .await?
            .json()
            .await?)
    }
}
