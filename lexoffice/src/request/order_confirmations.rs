use crate::model::OrderConfirmation;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<OrderConfirmation> {
    const ENDPOINT: &'static str = "order-confirmations";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::OrderConfirmation;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let order_confirmations = client.request::<OrderConfirmation>().by_id(uuid).await?;
/// println!("{:#?}", order_confirmations);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<OrderConfirmation> {}
