use crate::request::Request;
use crate::Error;
use crate::Result;
use derive_more::{Display, From, FromStr};
use reqwest::Method;
use reqwest::Url;
use typed_builder::TypedBuilder;

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Clone, Debug, Display, FromStr, From)]
pub struct ApiKey(String);

impl ApiKey {
    #[cfg(feature = "env")]
    pub fn from_env() -> Result<Self> {
        let env = std::env::var("LEXOFFICE_KEY")?;
        Ok(Self::from(env))
    }

    #[cfg(feature = "fs")]
    pub async fn from_file(file_name: &std::path::Path) -> Result<Self> {
        let contents = tokio::fs::read_to_string(file_name).await?;
        let contents = contents.trim();
        Ok(contents.parse().unwrap())
    }

    #[cfg(all(feature = "fs", feature = "env"))]
    pub async fn from_home() -> Result<Self> {
        use std::env;

        if let Some(home) = env::var_os("HOME") {
            let mut file_name = std::path::PathBuf::from(home);
            file_name.push(".lexoffice");

            Self::from_file(&file_name).await
        } else {
            Err(Error::HomeIsNotSet)
        }
    }

    pub async fn try_default() -> Result<Self> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "env")] {
                if let Ok(key) = Self::from_env() {
                    return Ok(key);
                }
            }
        }

        cfg_if::cfg_if! {
            if #[cfg(all(feature = "fs", feature = "env"))] {
                if let Ok(key) = Self::from_home().await {
                    return Ok(key);
                }
            }
        }

        Err(Error::FailedToLoadApiKey)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn default_client() -> reqwest::Client {
    let user_agent = concat!(
        "rust-",
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );

    reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn default_client() -> reqwest::Client {
    reqwest::Client::builder().build().unwrap()
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Client {
    api_key: ApiKey,

    #[builder(default=default_client())]
    http_client: reqwest::Client,

    #[builder(default=Url::parse(BASE_URL).unwrap())]
    base_url: Url,
}

impl Client {
    pub fn new(api_key: ApiKey) -> Self {
        Self::builder().api_key(api_key).build()
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

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}
