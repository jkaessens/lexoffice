use crate::model::CreditNote;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<CreditNote, ()> {
    const ENDPOINT: &'static str = "credit-notes";
}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::CreditNote;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let contacts = client.request::<CreditNote>().by_id(uuid).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<CreditNote, ()> {}
