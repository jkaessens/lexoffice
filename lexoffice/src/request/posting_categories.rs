use crate::model::PostingCategory;
use crate::request::Endpoint;
use crate::request::Request;
use crate::util::to_json_response;
use crate::Result;
use reqwest::Method;

impl Endpoint for Request<PostingCategory> {
    const ENDPOINT: &'static str = "posting-categories";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::PostingCategory;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let posting_categories = client.request::<PostingCategory>().get().await?;
/// println!("{:#?}", posting_categories);
/// Ok(())
/// # }
/// ```
///
impl Request<PostingCategory> {
    /// executes the request
    pub async fn get(self) -> Result<Vec<PostingCategory>> {
        let url = self.url();
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
