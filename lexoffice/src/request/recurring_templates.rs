use crate::model::RecurringTemplate;
use crate::request::impls::ById;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<RecurringTemplate> {
    const ENDPOINT: &'static str = "recurring-templates";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::RecurringTemplate;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("2b298bc7-8a5d-4bf1-9378-2267a117abc4")?;
/// let recurring_template = client.request::<RecurringTemplate>().by_id(uuid).await?;
/// println!("{:#?}", recurring_template);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<RecurringTemplate> {}
