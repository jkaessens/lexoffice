use bytes::Bytes;
use futures::io::Result;
use mime::Mime;
use std::ffi::OsStr;
use std::path::Path;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use tokio::stream::Stream;

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

pub fn guess_mime(path: &Path) -> mime::Mime {
    mime(path).clone()
}
pub fn guess_filename(mime: &Mime) -> std::string::String {
    format!("document.{}", extension(mime.as_ref()))
}

macro_rules! mime_table {
    ( $( $x:tt [ $y:tt $(, $z:tt)* ] ),* ) => {
            fn mime(path: &Path) -> &'static Mime {
                match path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                    .unwrap_or_else(String::new).as_str()
                {
                    $(
                        $y => &mime::$x,
                        $( $z => &mime::$x,)*
                    )*
                    _ => &mime::APPLICATION_OCTET_STREAM,
                }
            }

            fn extension(mime: &str) -> &'static str {
                $(
                    if mime == mime::$x {
                        concat!(".", $y)
                    } else
                )* {
                    ".xxx"
                }
            }
    };
}

mime_table!(
    APPLICATION_PDF ["pdf"],
    IMAGE_PNG ["png"],
    IMAGE_JPEG ["jpg", "jpeg"]
    );
