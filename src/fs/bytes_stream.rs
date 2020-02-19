use bytes::Bytes;
use futures::io::Result;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use tokio::stream::Stream;

#[derive(Debug)]
pub struct BytesStream<R: AsyncRead + Unpin + Send + Sync> {
    reader: R,
}

impl<R: AsyncRead + Unpin + Send + Sync> Stream for BytesStream<R> {
    type Item = Result<Bytes>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Bytes>>> {
        let mut buf = [0u8; 1024];

        match R::poll_read(Pin::new(&mut self.reader), cx, &mut buf) {
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(n)) => {
                Poll::Ready(Some(Ok(Bytes::copy_from_slice(&buf[0..n]))))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<R: AsyncRead + Unpin + Send + Sync> BytesStream<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}
