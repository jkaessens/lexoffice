use crate::error::LexOfficeError;
use crate::Result;
use mime::APPLICATION_JSON;
use reqwest::header::ACCEPT;
use reqwest::RequestBuilder;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn error_for_lexoffice(response: Response) -> Result<Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let message = response.json().await?;
        Err(LexOfficeError::new(status, message).into())
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

