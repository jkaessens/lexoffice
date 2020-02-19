pub mod contacts;
pub mod credit_notes;
pub mod event_subscriptions;
#[cfg(not(target_arch = "wasm32"))]
pub mod files;
pub mod invoices;
pub mod order_confirmations;
pub mod profile;
pub mod quotations;
pub mod voucher_list;

pub mod stream;

mod impls;
pub use impls::*;

use crate::client::Client;
use reqwest::Url;
use std::marker::PhantomData;

pub trait Endpoint {
    const ENDPOINT: &'static str;
}

#[derive(Clone, Debug)]
pub struct Request<T, S> {
    client: Client,
    url: Url,
    target: PhantomData<T>,
    state: PhantomData<S>,
}

impl<T, S> Request<T, S> {
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
