use crate::client::LoResponse;
use crate::error::Error;
use crate::model::files::FileContent;
use crate::model::files::TypeEnum;
use crate::model::server_resource::ServerResource;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::Requestable;
use crate::result::Result;
use crate::util::guess_filename;
use crate::util::guess_mime;
use crate::util::BytesStream;
use async_trait::async_trait;
use futures::future::TryFutureExt;
use futures::stream::TryStreamExt;
use mime::Mime;
use mime::APPLICATION_JSON;
use mime::APPLICATION_OCTET_STREAM;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Body;
use reqwest::Method;
use reqwest::Url;
use serde::Deserialize;
use std::convert::TryInto;
use std::path::Path;
use std::str::FromStr;
use tokio::fs;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct FileUploadResponse {
    id: Uuid,
}

#[async_trait]
pub trait FilesRequest {
    fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync;

    async fn by_id_str(self, uuid: &str) -> Result<ServerResource<File>>;

    async fn by_id<I>(self, uuid: I) -> Result<ServerResource<File>>
    where
        I: Into<Uuid> + Send + Sync;
    async fn upload<F>(self, file: F) -> Result<ServerResource<()>>
    where
        F: Into<File> + Send + Sync;
    async fn upload_path<P>(self, path: P) -> Result<ServerResource<()>>
    where
        P: AsRef<Path> + Send + Sync;
}

#[async_trait]
impl FilesRequest for Request<File> {
    fn by_id_url<I>(self: &Self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| Error::UrlCannotBeBase)?
            .push(&uuid.to_string());
        Ok(url)
    }

    async fn by_id_str(self, uuid: &str) -> Result<ServerResource<File>> {
        self.by_id(Uuid::parse_str(uuid)?).await
    }

    async fn by_id<I>(self, uuid: I) -> Result<ServerResource<File>>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let request = self.builder();
        let uuid: Uuid = uuid.into();
        let url = self.by_id_url(uuid)?;
        let response = request
            .request(Method::GET, &url)
            .send()
            .await?
            .error_for_legacy_lexoffice()
            .await?;
        let mime = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|x| x.to_str().ok())
            .and_then(|x| Mime::from_str(x).ok())
            .unwrap_or(APPLICATION_OCTET_STREAM);
        let stream = response.bytes_stream().err_into();
        let file = File::builder()
            .file(FileContent::Stream(Box::pin(stream)))
            .type_(TypeEnum::default())
            .mime(mime)
            .build();
        Ok(ServerResource {
            version: None,
            id: Some(uuid),
            inner: file,
        })
    }

    async fn upload<F>(self, file: F) -> Result<ServerResource<()>>
    where
        F: Into<File> + Send + Sync,
    {
        let file = file.into();
        let url = self.url();
        let form = Form::new()
            .part("file", file.try_into()?)
            .text("type", "voucher");
        let request = self
            .builder()
            .request(Method::POST, &url)
            .multipart(form)
            .header(ACCEPT, APPLICATION_JSON.as_ref());

        let response: FileUploadResponse = request
            .send()
            .await?
            .error_for_legacy_lexoffice()
            .await?
            .json()
            .await?;

        Ok(ServerResource {
            id: Some(response.id),
            version: None,
            inner: (),
        })
    }

    async fn upload_path<P>(self, path: P) -> Result<ServerResource<()>>
    where
        P: AsRef<Path> + Send + Sync,
    {
        self.upload(path.as_ref()).await
    }
}

impl TryInto<Part> for File {
    type Error = crate::error::Error;

    fn try_into(self) -> Result<Part> {
        let part = match self.file {
            FileContent::Bytes(bytes) => Part::stream(bytes),
            FileContent::Path(path) => {
                let stream = fs::File::open(path)
                    .map_ok(BytesStream::new)
                    .try_flatten_stream();
                Part::stream(Body::wrap_stream(stream))
            }
            FileContent::Stream(stream) => {
                Part::stream(Body::wrap_stream(stream))
            }
        };
        let file_name = guess_filename(&self.mime);
        Ok(part.mime_str(&self.mime.as_ref())?.file_name(file_name))
    }
}

impl From<&Path> for File {
    fn from(path: &Path) -> Self {
        Self::builder()
            .mime(guess_mime(path))
            .file(FileContent::Path(path.into()))
            .build()
    }
}

impl Endpoint for Request<File> {
    const ENDPOINT: &'static str = "files";
}
