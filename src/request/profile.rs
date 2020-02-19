use crate::model::Profile;
use crate::request::Endpoint;
use crate::request::Request;
use crate::util::to_json_response;
use crate::Result;
use reqwest::Method;

pub type ProfileRequest = Request<Profile, ()>;

impl Endpoint for ProfileRequest {
    const ENDPOINT: &'static str = "profile";
}

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
impl ProfileRequest {
    pub async fn get(self) -> Result<Profile> {
        let url = self.url();
        to_json_response(self.client().http_builder(Method::GET, url)).await
    }
}
