use crate::error::Error;
use crate::request::Request;
use crate::result::Result;
use reqwest::Method;
use reqwest::Url;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::read_to_string;
use typed_builder::TypedBuilder;

pub use crate::fs::ApiKeyFromFile;

static USER_AGENT: &str = concat!(
    "rust-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Clone, Debug)]
pub struct ApiKey(String);

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

#[derive(Debug, Clone, TypedBuilder)]
pub struct Client {
    api_key: ApiKey,

    #[builder(default=reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap())]
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
