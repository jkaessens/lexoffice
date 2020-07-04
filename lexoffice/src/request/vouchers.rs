use crate::model::Voucher;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<Voucher, ()> {
    const ENDPOINT: &'static str = "invoices";
}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::Voucher;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let invoices = client.request::<Voucher>().by_id(uuid).await?;
/// println!("{:#?}", invoices);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<Voucher, ()> {}
