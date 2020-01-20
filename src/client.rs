use crate::resource::*;
use crate::data::common::ServerResource;
use crate::data::common::Page;
use futures::stream::{Stream, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Method;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::error::Error;
use uuid::Uuid;

static USER_AGENT: &str = concat!(
    "rust ",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

static BASE_URL: &str = "https://api.lexoffice.io/v1";

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
}

impl Client {
    pub fn create(api_key: &str) -> Result<Self, Box<dyn Error>> {
        let mut header = HeaderMap::new();
        let header_value = format!("Bearer {}", api_key);
        header.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&header_value)?,
        );

        let http_client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(header)
            .build()?;

        Ok(Self { http_client })
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}/{}", BASE_URL, path);
        dbg!(&url);
        self.http_client.request(method, &url)
    }

    pub async fn json<O>(&self, path: &str) -> Result<O, Box<dyn Error>>
    where
        O: DeserializeOwned,
    {
        let response = self.request(Method::GET, &path).send().await?;
        {
        let response = self.request(Method::GET, &path).send().await?.text().await?;
        println!("{}", response);

        }
        Ok(response.json().await?)
    }

    pub async fn get<O>(&self) -> Result<ServerResource<O>, Box<dyn Error>>
    where
        O: SimpleResource + DeserializeOwned,
    {
        let path = O::BASE_PATH;
        self.json(&path).await
    }

    pub async fn page<O>(
        &self,
        page_number: usize,
    ) -> Result<Page<O>, Box<dyn Error>>
    where
        O: PaginatedResource + DeserializeOwned,
    {
        let path = format!("{}?page={}", O::BASE_PATH, page_number);
        self.json(&path).await
    }

    pub async fn by_id<O, U>(&self, id: U)  -> Result<ServerResource<O>, Box<dyn Error>>
    where
        O: ItemsResource + DeserializeOwned,
        U: Into<Uuid>,
    {
        let path = format!("{}/{}", O::BASE_PATH, id.into());
        self.json(&path).await
    }

    pub async fn stream<'a, O>(
        &'a self,
    ) -> Box<dyn 'a + Stream<Item = Result<Page<O>, Box<dyn Error>>>>
    where
        O: 'a + PaginatedResource + DeserializeOwned,
    {
        use futures::stream::iter;
        use std::iter::once;

        let first_page = match self.page::<O>(0).await {
            Ok(p) => p,
            Err(e) => return Box::new(iter(once(Err(e)))),
        };
        let total_pages = first_page.total_pages;

        let other_pages = iter(1..total_pages).then(move |n| self.page::<O>(n));

        Box::new(
            iter(once(Ok(first_page)))
                .chain(other_pages)
        )
    }
}
