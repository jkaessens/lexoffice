use reqwest::header::ACCEPT;
use mime::APPLICATION_JSON;
use crate::model::Profile;
use crate::reqwest_ext::ResponseExt;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::RequestTrait;
use crate::Result;
use reqwest::Method;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Profile;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let profile = client.request::<Profile>().get().await?;
/// println!("{:#?}", profile);
/// Ok(())
/// # }
/// ```
///
impl Request<Profile> {
    pub async fn get(self) -> Result<Profile>
    {
        let url = self.url();
        Ok(self
            .client()
            .http_builder(Method::GET, url)
            .header(ACCEPT, APPLICATION_JSON.as_ref())
            .send()
            .await?
            .error_for_lexoffice()
            .await?
            .json()
            .await?)
    }

}

impl Endpoint for Request<Profile> {
    const ENDPOINT: &'static str = "profile";
}
