use crate::error::Error;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::reqwest_ext::RequestBuilderExt;
use crate::reqwest_ext::ResponseExt;
use crate::result::Result;
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
        self.client
            .http_builder(Method::GET, url)
            .send()
            .await?
            .error_for_legacy_lexoffice()
            .await
    }

    pub async fn upload<P>(self, file_part: P) -> Result<FileResponse>
    where
        P: Into<Part> + Send + Sync,
    {
        let file_part = file_part.into();
        let url = self.url();
        let form = Form::new().part("file", file_part).text("type", "voucher");
        Ok(self
            .client()
            .http_builder(Method::POST, url)
            .multipart(form)
            .to_json_response()
            .await?)
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
