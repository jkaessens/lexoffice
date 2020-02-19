use crate::model::Invoice;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

pub type InvoiceRequest = Request<Invoice, ()>;

impl Endpoint for InvoiceRequest {
    const ENDPOINT: &'static str = "invoices";
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Invoice;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let invoices = client.request::<Invoice>().by_id(uuid).await?;
/// println!("{:#?}", invoices);
/// # Ok(())
/// # }
/// ```
///
impl ById for InvoiceRequest {}
