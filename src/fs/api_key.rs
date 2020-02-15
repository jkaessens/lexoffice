use crate::Error;
use crate::Result;
use async_trait::async_trait;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;

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
        let env = env::var("LEXOFFICE_KEY")?;
        Ok(Self::from(env))
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
