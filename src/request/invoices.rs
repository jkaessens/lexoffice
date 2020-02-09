use crate::model::Invoice;
use crate::request::ById;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::Invoice;
/// use lexoffice::request::ById;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let invoices = client.request::<Invoice>().by_id(uuid).await?;
/// println!("{:#?}", invoices);
/// # Ok(())
/// # }
/// ```
///
impl ById<Invoice> for Request<Invoice> {
}

impl Endpoint for Request<Invoice> {
    const ENDPOINT: &'static str = "invoices";
}
