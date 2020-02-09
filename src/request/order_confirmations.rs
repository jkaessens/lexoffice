use crate::model::OrderConfirmation;
use crate::request::ById;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::OrderConfirmation;
/// use lexoffice::request::ById;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let order_confirmations = client.request::<OrderConfirmation>().by_id(uuid).await?;
/// println!("{:#?}", order_confirmations);
/// # Ok(())
/// # }
/// ```
///
impl ById<OrderConfirmation> for Request<OrderConfirmation> {
}

impl Endpoint for Request<OrderConfirmation> {
    const ENDPOINT: &'static str = "order-confirmations";
}
