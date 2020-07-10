use crate::model::Quotation;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<Quotation, ()> {
    const ENDPOINT: &'static str = "quotations";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Quotation;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let quotations = client.request::<Quotation>().by_id(uuid).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<Quotation, ()> {}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Quotation;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let quotations = client.request::<Quotation>().page(0).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for Request<Quotation, ()> {}
