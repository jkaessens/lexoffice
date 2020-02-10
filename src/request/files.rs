use crate::model::files::FileContent;
use crate::model::files::TypeEnum;
use crate::model::server_resource::ServerResource;
use crate::model::File;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::Requestable;
use crate::Result;
use async_trait::async_trait;
use reqwest::Method;
use reqwest::Url;
use uuid::Uuid;

#[async_trait]
pub trait ById {
    fn by_id_url<I>(&self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync;

    async fn by_id_str(self, uuid: &str) -> Result<ServerResource<File>>;

    async fn by_id<I>(self, uuid: I) -> Result<ServerResource<File>>
    where
        I: Into<Uuid> + Send + Sync;
}

#[async_trait]
impl ById for Request<File> {
    fn by_id_url<I>(self: &Self, uuid: I) -> Result<Url>
    where
        I: Into<Uuid> + Send + Sync,
    {
        let uuid: Uuid = uuid.into();
        let mut url = self.url();
        url.path_segments_mut()
            .map_err(|_| "cannot be base")?
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
        let builder = self.builder();
        let uuid: Uuid = uuid.into();
        let url = self.by_id_url(uuid)?;
        let response = builder.request(Method::GET, &url).send().await?;
        let stream = response.bytes_stream();
        let file = File::builder()
            .type_(TypeEnum::Voucher)
            .file(FileContent::Stream(Box::new(stream)))
            .build();

        Ok(ServerResource {
            version: None,
            id: Some(uuid),
            inner: file,
        })
    }
}

impl Endpoint for Request<File> {
    const ENDPOINT: &'static str = "files";
}
