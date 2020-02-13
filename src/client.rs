use crate::error;
use crate::error::LexOfficeError;
use crate::request::Request;
use crate::result::Result;
use async_trait::async_trait;
use mime::APPLICATION_JSON;
use reqwest::header::ACCEPT;
use reqwest::Method;
use reqwest::Response;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::env;
use std::sync::Arc;

static USER_AGENT: &str = concat!(
    "rust-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Clone, Debug)]
pub struct ApiKey {
    key: String,
}

impl ApiKey {
    pub fn from_env() -> Result<Self> {
        let key = env::var("LEXOFFICE_KEY")?;
        Ok(Self { key })
    }
}

impl From<String> for ApiKey {
    fn from(key: String) -> ApiKey {
        Self { key }
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.key.fmt(fmt)
    }
}

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    api_key: ApiKey,
    http_client: reqwest::Client,
    pub base_url: Url,
}

impl RequestBuilder {
    pub fn request(
        &self,
        method: Method,
        url: &Url,
    ) -> reqwest::RequestBuilder {
        self.http_client
            .request(method, url.clone())
            .bearer_auth(&self.api_key)
    }

    pub async fn json<T>(&self, url: &Url) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self
            .request(Method::GET, url)
            .header(ACCEPT, APPLICATION_JSON.as_ref())
            .send()
            .await?
            .error_for_lexoffice()
            .await?
            .json()
            .await?)
    }
}

#[async_trait]
pub trait LoResponse
where
    Self: Sized,
{
    async fn error_for_legacy_lexoffice(self) -> Result<Self>;
    async fn error_for_lexoffice(self) -> Result<Self>;
}

#[async_trait]
impl LoResponse for Response {
    async fn error_for_legacy_lexoffice(self) -> Result<Self> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            Err(LexOfficeError::<error::LegacyMessage>::new(
                status,
                self.json().await?,
            )
            .into())
        }
    }
    async fn error_for_lexoffice(self) -> Result<Self> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            Err(LexOfficeError::<error::LegacyMessage>::new(
                status,
                self.json().await?,
            )
            .into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    request_builder: Arc<RequestBuilder>,
}

impl Client {
    pub fn new(api_key: ApiKey) -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        let base_url = Url::parse(BASE_URL).unwrap();

        Self {
            request_builder: Arc::new(RequestBuilder {
                base_url,
                http_client,
                api_key,
            }),
        }
    }

    pub fn request<T>(&self) -> Request<T> {
        Request::new(self.request_builder.clone())
    }
}
