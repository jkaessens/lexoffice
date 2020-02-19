//! This module allows making requests to the `order-confirmations` endpoint of the
//! Lexoffice API.

use crate::model::OrderConfirmation;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<OrderConfirmation, ()> {
    const ENDPOINT: &'static str = "order-confirmations";
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::OrderConfirmation;
///
/// # use lexoffice::Result;
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let order_confirmations = client.request::<OrderConfirmation>().by_id(uuid).await?;
/// println!("{:#?}", order_confirmations);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<OrderConfirmation, ()> {}
