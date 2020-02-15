use crate::model::CreditNote;
use crate::request::ById;
use crate::request::Endpoint;
use crate::request::Paginated;
use crate::request::Request;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, ApiKeyFromFile, Client};
/// use lexoffice::model::CreditNote;
/// use lexoffice::request::ById;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let contacts = client.request::<CreditNote>().by_id(uuid).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl ById<CreditNote> for Request<CreditNote> {}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, ApiKeyFromFile, Client};
/// use lexoffice::model::CreditNote;
/// use lexoffice::request::Paginated;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let contacts = client.request::<CreditNote>().page(0).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl Paginated<CreditNote> for Request<CreditNote> {}

impl Endpoint for Request<CreditNote> {
    const ENDPOINT: &'static str = "credit-notes";
}
