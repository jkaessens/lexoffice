use crate::error::Error;
use crate::request::Request;
use crate::result::Result;
use reqwest::Method;
use reqwest::Url;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::read_to_string;

static USER_AGENT: &str = concat!(
    "rust-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Clone, Debug)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn from_env() -> Result<Self> {
        let key = env::var("LEXOFFICE_KEY")?;
        Ok(Self(key))
    }

    pub async fn from_file(file_name: &Path) -> Result<Self> {
        let contents = read_to_string(file_name).await?;
        let key = contents.trim().to_string();
        Ok(Self(key))
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
        Self(key)
    }
}

impl std::fmt::Display for ApiKey {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.0.fmt(fmt)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    api_key: ApiKey,
    http_client: reqwest::Client,
    pub base_url: Url,
}

impl Client {
    pub fn new(api_key: ApiKey) -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .unwrap();
        let base_url = Url::parse(BASE_URL).unwrap();

        Self {
            base_url,
            http_client,
            api_key,
        }
    }

    pub fn request<T>(&self) -> Request<T> {
        Request::new(self.clone())
    }

    pub fn http_builder(
        &self,
        method: Method,
        url: Url,
    ) -> reqwest::RequestBuilder {
        self.http_client
            .request(method, url)
            .bearer_auth(&self.api_key)
    }
}
