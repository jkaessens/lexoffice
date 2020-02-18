use crate::error::Error;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::result::Result;
use crate::util::error_for_legacy_lexoffice;
use crate::util::to_json_response;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Method;
use reqwest::Response;
use reqwest::Url;
use serde::Deserialize;
use uuid::Uuid;

impl Endpoint for Request<File> {
    const ENDPOINT: &'static str = "files";
}

#[derive(Deserialize, Debug)]
pub struct FileResponse {
    pub id: Uuid,
}

impl Request<File> {
    pub fn by_id_url<I>(self: &Self, uuid: I) -> Result<Url>
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

    pub async fn by_id_str(self, uuid: &str) -> Result<Response> {
        self.by_id(Uuid::parse_str(uuid)?).await
    }

    pub async fn by_id<I>(self, uuid: I) -> Result<Response>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let url = self.by_id_url(uuid)?;
        error_for_legacy_lexoffice(
            self.client.http_builder(Method::GET, url).send().await?,
        )
        .await
    }

    pub async fn upload<P>(self, file_part: P) -> Result<FileResponse>
    where
        P: Into<Part> + Send + Sync,
    {
        let file_part = file_part.into();
        let url = self.url();
        let form = Form::new().part("file", file_part).text("type", "voucher");
        to_json_response(
            self.client()
                .http_builder(Method::POST, url)
                .multipart(form),
        )
        .await
    }

    #[cfg(feature = "fs")]
    pub async fn upload_path<P>(self, path: P) -> Result<FileResponse>
    where
        P: AsRef<std::path::Path> + Send + Sync,
    {
        use crate::mime::*;
        use reqwest::Body;

        let path = path.as_ref();
        let file = tokio::fs::File::open(path).await?;
        let stream = crate::fs::BytesStream::new(file);
        let mime = path.mime();
        let part = Part::stream(Body::wrap_stream(stream))
            .mime_str(mime.as_ref())?
            .file_name(format!("document.{}", mime.extension()));
        self.upload(part).await
    }
}
