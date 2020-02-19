use crate::model::Page;
use crate::request::stream::PageStream;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::to_json_response;
use reqwest::Method;
use serde::de::DeserializeOwned;

/// This trait marks a `Request` as `Paginated` and unlocks the
/// `Request::page()`, `Request::page_size()`, and `Request::stream()` methods.
///
/// For further reading please see
/// [the official API docs](https://developers.lexoffice.io/docs/#paging-of-resources)
pub trait Paginated {}

impl<T, S> Request<T, S>
where
    Self: Endpoint + Paginated + Sync + Send + Clone,
    T: DeserializeOwned + Sync + Send + 'static,
    S: Sync + Send,
{
    /// This method gets a page that contains items of type `T` from the API.
    /// It also allows to define a number of items to request per page.
    /// `Request<T>` must implement the `Paginated` trait in order to make
    /// this function available.
    pub async fn page_size(self, page: usize, size: usize) -> Result<Page<T>> {
        let mut url = self.url();
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair("size", &size.to_string());

        to_json_response(self.client().http_builder(Method::GET, url)).await
    }

    /// This method gets a page that contains items of type `T` from the API
    /// `Request<T>` must implement the `Paginated` trait in order to make
    /// this function available.
    pub async fn page(self, page: usize) -> Result<Page<T>> {
        let mut url = self.url();
        url.query_pairs_mut().append_pair("page", &page.to_string());

        to_json_response(self.client().http_builder(Method::GET, url)).await
    }

    /// Creates a `PageStream` from this request.
    pub fn stream(self) -> PageStream<T, S> {
        self.into()
    }
}
