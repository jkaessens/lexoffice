use reqwest::Method;
use reqwest::RequestBuilder;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize};
use std::error::Error;
use std::fmt;

static USER_AGENT: &str = concat!(
    "rust-",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        let api_key = api_key.to_string();
        let http_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .unwrap();

        Self {
            http_client,
            api_key,
        }
    }

    pub fn request<U>(&self, method: Method, url: U) -> RequestBuilder
    where
        U: Into<Url>,
    {
        self.http_client
            .request(method, url.into())
            .bearer_auth(&self.api_key)
    }
}
