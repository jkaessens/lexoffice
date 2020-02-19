use crate::model::Quotation;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;

pub type QuotationRequest = Request<Quotation, ()>;

impl Endpoint for QuotationRequest {
    const ENDPOINT: &'static str = "quotations";
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Quotation;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let quotations = client.request::<Quotation>().by_id(uuid).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl ById for QuotationRequest {}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Quotation;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let quotations = client.request::<Quotation>().page(0).await?;
/// println!("{:#?}", quotations);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for QuotationRequest {}
