use crate::error::Error;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::Requestable;
use crate::reqwest_ext::RequestBuilderExt;
use crate::reqwest_ext::ResponseExt;
use crate::result::Result;
use async_trait::async_trait;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use reqwest::Method;
use reqwest::Response;
use reqwest::Url;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct FileResponse {
    pub id: Uuid,
}

#[async_trait]
pub trait FilesRequest {
    fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync;

    async fn by_id_str(self, uuid: &str) -> Result<Response>;

    async fn by_id<I>(self, uuid: I) -> Result<Response>
    where
        I: Into<Uuid> + Send + Sync;
    async fn upload<P>(self, file_part: P) -> Result<FileResponse>
    where
        P: Into<Part> + Send + Sync;
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

    async fn by_id_str(self, uuid: &str) -> Result<Response> {
        self.by_id(Uuid::parse_str(uuid)?).await
    }

    async fn by_id<I>(self, uuid: I) -> Result<Response>
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

    async fn upload<P>(self, file_part: P) -> Result<FileResponse>
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
}

impl Endpoint for Request<File> {
    const ENDPOINT: &'static str = "files";
}
