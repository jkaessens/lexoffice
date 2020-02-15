use crate::fs::BytesStream;
use crate::mime::ExtensionExt;
use crate::mime::MimeExt;
use crate::model::files;
use crate::request::files::FileResponse;
use crate::request::files::FilesRequest;
use crate::request::Request;
use crate::Result;
use async_trait::async_trait;
use reqwest::multipart::Part;
use reqwest::Body;
use std::path::Path;
use tokio::fs::File;

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
        let mime = path.mime();
        let part = Part::stream(Body::wrap_stream(stream))
            .mime_str(mime.as_ref())?
            .file_name(format!("document.{}", mime.extension()));
        self.upload(part).await
    }
}
