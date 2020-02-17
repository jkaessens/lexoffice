use crate::request::Request;
use derive_more::{Display, From, FromStr};
use reqwest::Method;
use reqwest::Url;
use std::env;
use typed_builder::TypedBuilder;

static USER_AGENT: &str = concat!(
    "rust-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Clone, Debug, Display, FromStr, From)]
pub struct ApiKey(String);

#[cfg(not(target_arch = "wasm32"))]
fn default_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn default_client() -> reqwest::Client {
    reqwest::Client::builder()
        .build()
        .unwrap()
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
