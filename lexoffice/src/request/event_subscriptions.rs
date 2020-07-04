use crate::model::EventSubscription;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;

impl Endpoint for Request<EventSubscription, ()> {
    const ENDPOINT: &'static str = "event-subscriptions";
}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::EventSubscription;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let event_subscriptions = client.request::<EventSubscription>().by_id(uuid).await?;
/// println!("{:#?}", event_subscriptions);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<EventSubscription, ()> {}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::EventSubscription;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let event_subscriptions = client.request::<EventSubscription>().page(0).await?;
/// println!("{:#?}", event_subscriptions);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for Request<EventSubscription, ()> {}
