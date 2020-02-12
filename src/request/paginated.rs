use crate::error::Error;
use crate::model::server_resource::ServerResource;
use crate::model::Page;
use crate::request::Requestable;
use crate::result::Result;
use async_trait::async_trait;
use futures::stream::Stream;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::ops::Range;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::vec::IntoIter;

#[async_trait]
pub trait Paginated<T>
where
    Self: Requestable + Sized,
    T: DeserializeOwned,
{
    async fn page_size(self, page: usize, size: usize) -> Result<Page<T>>
    where
        T: 'async_trait,
    {
        let mut url = self.url();
        url.query_pairs_mut()
            .append_pair("page", &page.to_string())
            .append_pair("size", &size.to_string());

        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }

    async fn page(self, page: usize) -> Result<Page<T>>
    where
        T: 'async_trait,
    {
        let mut url = self.url();
        url.query_pairs_mut().append_pair("page", &page.to_string());

        let builder = self.builder();
        Ok(builder.json(&url).await?)
    }

    fn stream(self) -> PageStream<Self, T>
    where
        Self: Clone + Sync + Send,
    {
        self.into()
    }
}

type FutureType<T> = dyn Future<Output = Result<Page<T>>> + Send;
pub struct PageStream<R, T>
where
    R: Paginated<T> + 'static,
    T: DeserializeOwned + Sized + 'static,
{
    request: R,
    future: Option<Pin<Box<FutureType<T>>>>,
    pages: Option<Range<usize>>,
    iter: Option<IntoIter<ServerResource<T>>>,
}

impl<R, T> From<R> for PageStream<R, T>
where
    R: Paginated<T> + Sync + Send + Clone,
    T: DeserializeOwned,
{
    fn from(request: R) -> Self {
        let request_clone = request.clone();

        Self {
            request,
            future: Some(request_clone.page(0)),
            pages: None,
            iter: None,
        }
    }
}

impl<R, T> PageStream<R, T>
where
    R: Paginated<T> + Unpin + Sync + Send + Clone,
    T: DeserializeOwned + Unpin,
{
    fn poll_item(&mut self) -> Option<ServerResource<T>> {
        self.iter.as_mut().and_then(|x| x.next())
    }

    fn poll_future(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<ServerResource<T>>>> {
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
    ) -> Poll<Option<Result<ServerResource<T>>>> {
        let request = self.request.clone();
        let pages = self.pages.get_or_insert(1..page.total_pages);
        self.future = pages.next().map(|page| request.page(page));

        let mut iter = page.content.into_iter();
        let first_item = iter.next().map(Ok);
        self.iter = Some(iter);

        Poll::Ready(first_item)
    }

    fn on_error(
        mut self: Pin<&mut Self>,
        err: Error,
    ) -> Poll<Option<Result<ServerResource<T>>>> {
        self.future = None;
        self.pages = None;
        Poll::Ready(Some(Err(err)))
    }
}

impl<R, T> Stream for PageStream<R, T>
where
    R: Paginated<T> + Unpin + Sync + Send + Clone,
    T: DeserializeOwned + Unpin,
{
    type Item = Result<ServerResource<T>>;

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
