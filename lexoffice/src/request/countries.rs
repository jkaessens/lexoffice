use crate::model::Country;
use crate::request::Endpoint;
use crate::request::Request;
use crate::util::to_json_response;
use crate::Result;
use reqwest::Method;

impl Endpoint for Request<Country> {
    const ENDPOINT: &'static str = "countries";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Country;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let countries = client.request::<Country>().get().await?;
/// println!("{:#?}", countries);
/// Ok(())
/// # }
/// ```
///
impl Request<Country> {
    /// executes the request
    pub async fn get(self) -> Result<Vec<Country>> {
        let url = self.url();
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
