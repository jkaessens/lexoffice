use crate::model::Page;
use crate::request::stream::PageStream;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use reqwest::Method;
use serde::de::DeserializeOwned;

pub trait Paginated {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Paginated + Sync + Send + Clone,
    T: DeserializeOwned + Sync + Send + 'static,
    S: Sync + Send,
{
    pub async fn page_size(self, page: usize, size: usize) -> Result<Page<T>> {
        let mut url = self.url();
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair("size", &size.to_string());

        to_json_response(self.client().http_builder(Method::GET, url)).await
    }

    pub async fn page(self, page: usize) -> Result<Page<T>> {
        let mut url = self.url();
        url.query_pairs_mut().append_pair("page", &page.to_string());

        to_json_response(self.client().http_builder(Method::GET, url)).await
    }

    pub fn stream(self) -> PageStream<T, S> {
        self.into()
    }
}
