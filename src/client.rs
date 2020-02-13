use crate::error;
use crate::error::Error;
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
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::read_to_string;

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

    pub async fn from_file(file_name: &Path) -> Result<Self> {
        let contents = read_to_string(file_name).await?;
        let key = contents.trim().to_string();
        Ok(Self { key })
    }

    pub async fn from_home() -> Result<Self> {
        if let Some(home) = env::var_os("HOME") {
            let mut file_name = PathBuf::from(home);
            file_name.push(".lexoffice");

            Self::from_file(&file_name).await
        } else {
            Err(Error::HomeIsNotSet)
        }
    }

    pub async fn try_default() -> Result<Self> {
        if let Ok(key) = Self::from_env() {
            Ok(key)
        } else if let Ok(key) = Self::from_home().await {
            Ok(key)
        } else {
            Err(Error::FailedToLoadApiKey)
        }
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
