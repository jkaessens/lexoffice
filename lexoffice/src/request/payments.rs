use crate::model::Payment;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<Payment> {
    const ENDPOINT: &'static str = "payments";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Payment;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("a886c776-c1b2-427d-999e-a687f688da08")?;
/// let payment = client.request::<Payment>().by_id(uuid).await?;
/// println!("{:#?}", payment);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<Payment> {}
