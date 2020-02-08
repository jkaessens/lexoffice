use reqwest::StatusCode;
use serde::Deserialize;
use derive_more::Display;

#[derive(Debug, Deserialize, Display)]
pub struct JsonMessage {
    message: String,
}

#[derive(Debug, Display)]
#[display(fmt = "HTTP Error {}: {}", status, message)]
pub struct HttpJsonError {
    status: StatusCode,
    message: JsonMessage,
}

impl HttpJsonError {
    pub fn new(status: StatusCode, message: JsonMessage) -> Self {
        Self { status, message }
    }
}

impl std::error::Error for HttpJsonError {}
