//! module that handles requests to the Lexoffice API
mod contacts;
mod credit_notes;
mod event_subscriptions;
#[cfg(not(target_arch = "wasm32"))]
mod files;
mod invoices;
mod order_confirmations;
mod profile;
mod quotations;
mod voucher_list;

pub mod stream;

mod impls;
pub use impls::*;
pub use voucher_list::*;

use crate::client::Client;
use reqwest::Url;
use std::marker::PhantomData;

/// This crate is needed in order to build an URL for a request. It is implemented by
/// all Request variants that allow Requests to the API.
///
/// If you want to reach an endpoint
/// `https://api.lexoffice.io/foobar` you must implement this trait as follows:
///
/// ```compile_fail
/// use lexoffice::request::Request;
/// use lexoffice::request::Endpoint;
///
/// // Model
/// struct Foobar {
///     // ...
/// }
/// impl Endpoint for Request<Foobar, ()> {
///     const ENDPOINT: &'static str = "foobar";
/// }
/// ```
pub trait Endpoint {
    /// The endpoint of a request.
    const ENDPOINT: &'static str;
}

/// Represents a request to the lexoffice API. Please note that in order to
/// create a working `Request` the Type variable `T` must allow requests to be
/// made. The type variable `S` is an optional argument that allows to share state
#[derive(Clone, Debug)]
pub struct Request<T, S> {
    client: Client,
    url: Url,
    target: PhantomData<T>,
    state: PhantomData<S>,
}

impl<T, S> Request<T, S> {
    /// Creates a new Request based with a Client
    pub fn new(client: Client) -> Self {
        let url = client.base_url().clone();
        Self {
            client,
            url,
            target: PhantomData,
            state: PhantomData,
        }
    }
}

impl<T, S> Request<T, S>
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
