use crate::model::EventSubscription;
use crate::request::ById;
use crate::request::Paginated;
use crate::request::Request;
use crate::request::Endpoint;

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::EventSubscription;
/// use lexoffice::request::ById;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let event_subscriptions = client.request::<EventSubscription>().by_id(uuid).await?;
/// println!("{:#?}", event_subscriptions);
/// # Ok(())
/// # }
/// ```
///
impl ById<EventSubscription> for Request<EventSubscription> {
}

/// # Examples
///
/// ``` no_run
/// use lexoffice::client::{ Client, ApiKey };
/// use lexoffice::model::EventSubscription;
/// use lexoffice::request::Paginated;
/// 
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let event_subscriptions = client.request::<EventSubscription>().page(0).await?;
/// println!("{:#?}", event_subscriptions);
/// # Ok(())
/// # }
/// ```
///
impl Paginated<EventSubscription> for Request<EventSubscription> {
}

impl Endpoint for Request<EventSubscription> {
    const ENDPOINT: &'static str = "event-subscriptions";
}
