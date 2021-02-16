use crate::model::Countries;
use crate::request::Endpoint;
use crate::request::Request;
use crate::util::to_json_response;
use crate::Result;
use reqwest::Method;

impl Endpoint for Request<Profile> {
    const ENDPOINT: &'static str = "profile";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Profile;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let profile = client.request::<Profile>().get().await?;
/// println!("{:#?}", profile);
/// Ok(())
/// # }
/// ```
///
impl Request<Profile> {
    /// executes the request
    pub async fn get(self) -> Result<Profile> {
        let url = self.url();
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
