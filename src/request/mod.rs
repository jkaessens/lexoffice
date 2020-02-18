mod contacts;
mod credit_notes;
mod event_subscriptions;
mod files;
mod impls;
mod invoices;
mod order_confirmations;
mod profile;
mod quotations;
mod voucher_list;
use crate::client::Client;
use reqwest::Url;
use std::marker::PhantomData;

pub trait Endpoint {
    const ENDPOINT: &'static str;
}

pub type Request<T> = StateRequest<T, ()>;
#[derive(Clone, Debug)]
pub struct StateRequest<T, S> {
    client: Client,
    url: Url,
    target: PhantomData<T>,
    state: PhantomData<S>,
}

impl<T, S> StateRequest<T, S> {
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

impl<T, S> StateRequest<T, S>
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
