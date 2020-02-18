use crate::error::LegacyMessage;
use crate::error::LexOfficeError;
use crate::error::Message;
use crate::Result;
use mime::APPLICATION_JSON;
use reqwest::header::ACCEPT;
use reqwest::RequestBuilder;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[allow(dead_code)]
pub async fn error_for_legacy_lexoffice(
    response: Response,
) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let message = response.json().await?;
        Err(LexOfficeError::<LegacyMessage>::new(status, message).into())
    }
}

pub async fn error_for_lexoffice(response: Response) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let message = response.json().await?;
        Err(LexOfficeError::<Message>::new(status, message).into())
    }
}

pub async fn to_json_response<T>(request: RequestBuilder) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(error_for_lexoffice(
        request
            .header(ACCEPT, APPLICATION_JSON.as_ref())
            .send()
            .await?,
    )
    .await?
    .json()
    .await?)
}
