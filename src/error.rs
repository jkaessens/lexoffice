use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct JsonMessage {
    message: String,
}

impl fmt::Display for JsonMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct Error {
    message: JsonMessage,
    status: StatusCode,
}

impl Error {
    pub fn new(status: StatusCode, message: JsonMessage) -> Error {
        Self { status, message }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexOffice HTTP Error {}: {}", self.status, self.message)
    }
}
