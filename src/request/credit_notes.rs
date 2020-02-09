use crate::model::CreditNote;
use crate::request::ById;
use crate::request::Paginated;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::CreditNote;
/// use lexoffice::request::ById;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let contacts = client.request::<CreditNote>().by_id(uuid).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl ById<CreditNote> for Request<CreditNote> {
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::CreditNote;
/// use lexoffice::request::Paginated;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let contacts = client.request::<CreditNote>().page(0).await?;
/// println!("{:#?}", contacts);
/// # Ok(())
/// # }
/// ```
///
impl Paginated<CreditNote> for Request<CreditNote> {
}

impl Endpoint for Request<CreditNote> {
    const ENDPOINT: &'static str = "credit-notes";
}
