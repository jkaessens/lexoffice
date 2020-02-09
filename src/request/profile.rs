use crate::model::Profile;
use crate::request::Simple;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::Profile;
/// use lexoffice::request::Simple;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let profile = client.request::<Profile>().get().await?;
/// println!("{:#?}", profile);
/// Ok(())
/// # }
/// ```
///
impl Simple<Profile> for Request<Profile> {
}

impl Endpoint for Request<Profile> {
    const ENDPOINT: &'static str = "profile";
}
