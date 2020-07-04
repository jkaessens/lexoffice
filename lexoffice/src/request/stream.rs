//! This module provides functionality to stream page items.

use crate::error::Error;
use crate::model::Page;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use futures::stream::Stream;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::ops::Range;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::vec::IntoIter;

#[cfg(target_arch = "wasm32")]
type FutureType<T> = dyn Future<Output = Result<Page<T>>>;
#[cfg(not(target_arch = "wasm32"))]
type FutureType<T> = dyn Future<Output = Result<Page<T>>> + Send;

/// Stream that allows to view multiple pages as contiguous stream of Page items `T`.
pub struct PageStream<T, S>
where
    Request<T, S>: Paginated + Clone + Endpoint,
    T: DeserializeOwned,
    S: Sync + Send + 'static,
{
    request: Request<T, S>,
    future: Option<Pin<Box<FutureType<T>>>>,
    pages: Option<Range<usize>>,
    iter: Option<IntoIter<T>>,
}

impl<T, S> From<Request<T, S>> for PageStream<T, S>
where
    Request<T, S>: Paginated + Clone + Endpoint,
    T: DeserializeOwned + Sync + Send + 'static,
    S: Sync + Send + 'static,
{
    fn from(request: Request<T, S>) -> Self {
        let request_clone = request.clone();

        Self {
            request,
            future: Some(Box::pin(request_clone.page(0))),
            pages: None,
            iter: None,
        }
    }
}

impl<T, S> PageStream<T, S>
where
    Request<T, S>: Endpoint + Paginated + Unpin + Sync + Send + Clone,
    T: DeserializeOwned + Unpin + Sync + Send + 'static,
    S: Sync + Send + 'static,
{
    fn poll_item(&mut self) -> Option<T> {
        self.iter.as_mut().and_then(|x| x.next())
    }

    fn poll_future(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<T>>> {
        if let Some(future) = self.future.as_mut() {
            match Pin::new(future).poll(cx) {
                Poll::Ready(Ok(page)) => self.on_new_page(page),
                Poll::Ready(Err(err)) => self.on_error(err),
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        }
    }

    fn on_new_page(
        mut self: Pin<&mut Self>,
        page: Page<T>,
    ) -> Poll<Option<Result<T>>> {
        let request = self.request.clone();
        let pages = self.pages.get_or_insert(1..page.total_pages);
        self.future = match pages.next() {
            Some(x) => Some(Box::pin(request.page(x))),
            None => None,
        };

        let mut iter = page.content.into_iter();
        let first_item = iter.next().map(Ok);
        self.iter = Some(iter);

        Poll::Ready(first_item)
    }

    fn on_error(
        mut self: Pin<&mut Self>,
        err: Error,
    ) -> Poll<Option<Result<T>>> {
        self.future = None;
        self.pages = None;
        Poll::Ready(Some(Err(err)))
    }
}

impl<T, S> Stream for PageStream<T, S>
where
    Request<T, S>: Endpoint + Paginated + Unpin + Sync + Send + Clone,
    T: DeserializeOwned + Unpin + Send + Sync + 'static,
    S: Sync + Send + 'static,
{
    type Item = Result<T>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some(item) = self.poll_item() {
            Poll::Ready(Some(Ok(item)))
        } else {
            self.poll_future(cx)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!();
    }
}
