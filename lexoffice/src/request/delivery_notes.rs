use std::marker::PhantomData;

use crate::model::DeliveryNote;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::impls::Storable;
use crate::request::impls::Updatable;
use crate::request::Endpoint;
use crate::request::Request;
use uuid::Uuid;

use super::RequestWithState;

impl Request<DeliveryNote> {
    /// To be able to pursue a sales voucher to a delivery note, the optional
    /// query parameter precedingSalesVoucherId needs to be set. The id value
    /// `id` refers to the preceding sales voucher which is going to be
    /// pursued.
    pub async fn pursue<U>(
        mut self,
        id: U,
    ) -> RequestWithState<DeliveryNote, Uuid>
    where
        U: Into<Uuid>,
    {
        let id = id.into().to_string();
        self.url
            .query_pairs_mut()
            .append_pair("precedingSalesVoucherId", &id);
        RequestWithState {
            client: self.client,
            url: self.url,
            target: self.target,
            state: PhantomData,
        }
    }
}

impl Endpoint for Request<DeliveryNote> {
    const ENDPOINT: &'static str = "delivery-notes";
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::DeliveryNote;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("1195100a-7c44-40f0-972f-cef2844bc2ef")?;
/// let delivery_note = client.request::<DeliveryNote>().by_id(uuid).await?;
/// println!("{:#?}", delivery_note);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<DeliveryNote> {}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::DeliveryNote;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let delivery_note = client.request::<DeliveryNote>().page(0).await?;
/// println!("{:#?}", delivery_note);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for Request<DeliveryNote> {}

/// TODO doc
impl Storable for Request<DeliveryNote> {}

/// TODO doc
impl Updatable for Request<DeliveryNote> {}
