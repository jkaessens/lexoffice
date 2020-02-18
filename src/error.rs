use derive_error::Error;
use derive_more::Display;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use std::fmt;

#[derive(Debug, Deserialize, Display)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Message {
    message: String,
}

#[derive(Debug, Deserialize, Display)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
#[display(fmt = "{:#?}", issue_list)]
pub struct LegacyMessage {
    issue_list: Value,
}

#[derive(Debug, Error)]
pub enum Error {
    LexOfficeLegacy(LexOfficeError<LegacyMessage>),
    LexOffice(LexOfficeError<Message>),
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Uuid(uuid::Error),
    Env(std::env::VarError),
    UrlCannotBeBase,
    FailedToLoadApiKey,
    HomeIsNotSet,
}

impl Unpin for Error {}

#[derive(Debug, Display)]
#[display(fmt = "HTTP Error {}: {}", status, message)]
pub struct LexOfficeError<T> {
    status: StatusCode,
    message: T,
}

impl<T> LexOfficeError<T> {
    pub fn new(status: StatusCode, message: T) -> Self {
        Self { status, message }
    }
}

impl<T> std::error::Error for LexOfficeError<T> where
    T: fmt::Display + fmt::Debug
{
}
