//! module that handles requests to the Lexoffice API
mod contacts;
mod countries;
mod credit_notes;
mod event_subscriptions;
#[cfg(not(target_arch = "wasm32"))]
mod files;
mod invoices;
mod order_confirmations;
mod payments;
mod profile;
mod quotations;
mod voucherlist;
mod vouchers;

pub mod stream;

mod impls;
pub use impls::*;
pub use voucherlist::*;

use crate::client::Client;
use crate::marker::ReadOnly;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

/// This struct is returned when an object has changed
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultInfo<T> {
    /// The id of the changed object
    pub id: uuid::Uuid,
    /// The URI of the changed object
    pub resource_uri: String,
    /// The creation time of the changed object
    pub created_date: chrono::DateTime<chrono::Utc>,
    /// The update time of the changed object
    pub updated_date: chrono::DateTime<chrono::Utc>,
    /// The new version of the changed object
    pub version: u64,
    #[serde(skip)]
    result_for: PhantomData<T>,
}

/// This crate is needed in order to build an URL for a request. It is implemented by
/// all Request variants that allow Requests to the API.
///
/// If you want to reach an endpoint
/// `https://api.lexoffice.io/foobar` you must implement this trait as follows:
///
/// ```compile_fail
/// use lexoffice::request::{Request, Endpoint};
///
/// // Model
/// struct FooBar {
///     // ...
/// }
/// impl Endpoint for Request<FooBar> {
///     const ENDPOINT: &'static str = "foobar";
/// }
/// ```
pub trait Endpoint {
    /// The endpoint of a request.
    const ENDPOINT: &'static str;
}

/// Represents a request to the lexoffice API. Please note that in order to
/// create a working `Request` the Type variable `T` must allow requests to be
/// made.
pub type Request<T> = RequestWithState<T, ()>;

/// Represents a request to the lexoffice API. Please note that in order to
/// create a working `Request` the Type variable `T` must allow requests to be
/// made. The type variable `S` is an optional argument that allows to share
/// state of the time, for example to make sure that certain functions have
/// been called.
#[derive(Clone, Debug)]
pub struct RequestWithState<T: Clone, S: Clone> {
    client: Client,
    url: Url,
    target: PhantomData<T>,
    state: PhantomData<S>,
}

/// Represents type with an id.
pub trait HasId {
    /// gets the id from an object.
    fn id(&self) -> &ReadOnly<Uuid>;
}

impl<T: Clone, S: Clone> RequestWithState<T, S> {
    /// Creates a new Request based with a Client
    pub fn new(client: Client) -> Self {
        let url = client.base_url().clone();
        RequestWithState {
            client,
            url,
            target: PhantomData,
            state: PhantomData,
        }
    }
}

impl<T: Clone, S: Clone> RequestWithState<T, S>
where
    Self: Endpoint,
{
    fn url(&self) -> Url {
        let mut url = self.url.clone();
        url.path_segments_mut().unwrap().push(Self::ENDPOINT);
        url
    }

    fn client(&self) -> &Client {
        &self.client
    }
}
