use std::marker::PhantomData;

use crate::model::Dunning;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::impls::Storable;
use crate::request::impls::Updatable;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::RequestWithState;
use uuid::Uuid;

impl Endpoint for Request<Dunning> {
    const ENDPOINT: &'static str = "dunnings";
}

impl Request<Dunning> {
    /// To be able to pursue a sales voucher to a dunning, the optional query
    /// parameter precedingSalesVoucherId needs to be set. The id value `id`
    /// refers to the preceding sales voucher which is going to be pursued.
    pub async fn pursue<U>(mut self, uuid: U) -> RequestWithState<Dunning, Uuid>
    where
        U: Into<Uuid>,
    {
        let uuid = uuid.into().to_string();
        self.url
            .query_pairs_mut()
            .append_pair("precedingSalesVoucherId", &uuid);
        RequestWithState {
            client: self.client,
            url: self.url,
            target: self.target,
            state: PhantomData,
        }
    }
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Dunning;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("1195100a-7c44-40f0-972f-cef2844bc2ef")?;
/// let delivery_note = client.request::<Dunning>().by_id(uuid).await?;
/// println!("{:#?}", delivery_note);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<Dunning> {}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Dunning;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let delivery_note = client.request::<Dunning>().page(0).await?;
/// println!("{:#?}", delivery_note);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for Request<Dunning> {}

/// TODO doc
impl Storable for RequestWithState<Dunning, Uuid> {}

/// TODO doc
impl Storable for Request<Dunning> {}

/// TODO doc
impl Updatable for Request<Dunning> {}
