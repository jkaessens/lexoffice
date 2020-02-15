mod mime;

use crate::fs::mime::extension;
use crate::fs::mime::mime;
use crate::model::files;
use crate::request::files::FileResponse;
use crate::request::files::FilesRequest;
use crate::request::Request;
use crate::util::BytesStream;
use crate::Error;
use crate::Result;
use async_trait::async_trait;
use reqwest::multipart::Part;
use reqwest::Body;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
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
        let mime = mime(path).as_ref();
        let part = Part::stream(Body::wrap_stream(stream))
            .mime_str(mime)?
            .file_name(format!("document.{}", extension(mime)));
        self.upload(part).await
    }
}

#[async_trait]
pub trait ApiKeyFromFile
where
    Self: Sized,
{
    fn from_env() -> Result<Self>;

    async fn from_file(file_name: &Path) -> Result<Self>;

    async fn from_home() -> Result<Self>;

    async fn try_default() -> Result<Self>;
}

#[async_trait]
impl ApiKeyFromFile for crate::client::ApiKey {
    fn from_env() -> Result<Self> {
        let key = env::var("LEXOFFICE_KEY")?;
        Ok(Self::from(key))
    }

    async fn from_file(file_name: &Path) -> Result<Self> {
        let contents = fs::read_to_string(file_name).await?;
        let contents = contents.trim();
        Ok(contents.parse().unwrap())
    }

    async fn from_home() -> Result<Self> {
        if let Some(home) = env::var_os("HOME") {
            let mut file_name = PathBuf::from(home);
            file_name.push(".lexoffice");

            Self::from_file(&file_name).await
        } else {
            Err(Error::HomeIsNotSet)
        }
    }

    async fn try_default() -> Result<Self> {
        if let Ok(key) = Self::from_env() {
            Ok(key)
        } else if let Ok(key) = Self::from_home().await {
            Ok(key)
        } else {
            Err(Error::FailedToLoadApiKey)
        }
    }
}
