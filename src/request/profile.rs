use crate::model::Profile;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::Simple;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, ApiKeyFromFile, Client};
/// use lexoffice::model::Profile;
/// use lexoffice::request::Simple;
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
impl Simple<Profile> for Request<Profile> {}

impl Endpoint for Request<Profile> {
    const ENDPOINT: &'static str = "profile";
}
