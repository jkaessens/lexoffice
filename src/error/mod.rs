use reqwest::StatusCode;
use serde::Deserialize;
use derive_more::Display;

#[derive(Debug, Deserialize, Display)]
pub struct JsonMessage {
    message: String,
}

#[derive(Debug, Display)]
#[display(fmt = "LexOfficeError {}: {}", status, message)]
pub struct LexOfficeError {
    status: StatusCode,
    message: JsonMessage,
}

impl LexOfficeError {
    pub fn new(status: StatusCode, message: JsonMessage) -> Self {
        Self { status, message }
    }
}

impl std::error::Error for LexOfficeError {}
