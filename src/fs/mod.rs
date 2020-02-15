mod mime;

use crate::model::files;
use crate::request::files::FileResponse;
use crate::request::Request;
use crate::Result;
use std::path::Path;
use tokio::fs::File;
//use crate::request::files::FilesRequest;
use crate::fs::mime::extension;
use crate::fs::mime::mime;
use crate::request::files::FilesRequest;
use crate::util::BytesStream;
use async_trait::async_trait;
use reqwest::multipart::Part;
use reqwest::Body;

#[async_trait]
pub trait FilesUpload
where
    Self: Sized,
{
    async fn upload_path<P>(self, path: P) -> Result<FileResponse>
    where
        P: AsRef<Path> + Send + Sync;
}

#[async_trait]
impl FilesUpload for Request<files::File> {
    async fn upload_path<P>(self, path: P) -> Result<FileResponse>
    where
        P: AsRef<Path> + Send + Sync,
    {
        let path = path.as_ref();
        let file = File::open(path).await?;
        let stream = BytesStream::new(file);
        let mime = mime(path).as_ref();
        let part = Part::stream(Body::wrap_stream(stream))
            .mime_str(mime)?
            .file_name(format!("document.{}", extension(mime)));
        self.upload(part).await
    }
}
