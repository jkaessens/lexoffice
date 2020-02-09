use crate::request::Request;
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::env;
use std::error::Error;
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
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let key = env::var("LEXOFFICE_KEY")?;
        Ok(Self { key })
    }

    pub async fn from_file(
        file_name: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = read_to_string(file_name).await?;
        let key = contents.trim().to_string();
        Ok(Self { key })
    }

    pub async fn from_home() -> Result<Self, Box<dyn std::error::Error>> {
        match env::var_os("HOME") {
            Some(home) => {
                let mut file_name = PathBuf::from(home);
                file_name.push(".lexoffice");

                Self::from_file(&file_name).await
            }
            None => Err("HOME is not set".into()),
        }
    }

    pub async fn try_default() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(key) = Self::from_env() {
            Ok(key)
        } else if let Ok(key) = Self::from_home().await {
            Ok(key)
        } else {
            Err("failed to load API Key".into())
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
    pub fn request<U>(&self, method: Method, url: &U) -> reqwest::RequestBuilder
    where
        U: Into<Url> + Clone,
    {
        self.http_client
            .request(method, url.clone().into())
            .bearer_auth(&self.api_key)
    }

    pub async fn json<U, T>(&self, url: &U) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
        U: Into<Url> + Clone,
    {
        Ok(self
            .request(Method::GET, url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
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
