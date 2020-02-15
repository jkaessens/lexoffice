use crate::error::LegacyMessage;
use crate::error::LexOfficeError;
use crate::error::Message;
use crate::Result;
use async_trait::async_trait;
use reqwest::Response;

#[async_trait]
pub trait ResponseExt
where
    Self: Sized,
{
    async fn error_for_legacy_lexoffice(self) -> Result<Self>;
    async fn error_for_lexoffice(self) -> Result<Self>;
}

#[async_trait]
impl ResponseExt for Response {
    async fn error_for_legacy_lexoffice(self) -> Result<Self> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            Err(LexOfficeError::<LegacyMessage>::new(
                status,
                self.json().await?,
            )
            .into())
        }
    }
    async fn error_for_lexoffice(self) -> Result<Self> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            Err(LexOfficeError::<Message>::new(status, self.json().await?)
                .into())
        }
    }
}
