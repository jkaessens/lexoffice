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

/// The Errors that may occur working with this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Legacy Errors from the LexOffice API
    ///
    /// See
    /// [The official API](https://developers.lexoffice.io/docs/#error-codes-legacy-error-response)
    /// for more information.
    LexOfficeLegacy(LexOfficeError<LegacyMessage>),

    /// Errors from the LexOffice API
    ///
    /// See
    /// [The official API](https://developers.lexoffice.io/docs/#error-codes)
    /// for more information.
    LexOffice(LexOfficeError<Message>),

    /// I/O Errors
    Io(std::io::Error),

    /// Errors from the `reqwest` crate.
    Reqwest(reqwest::Error),

    /// Errors from the `uuid` crate.
    Uuid(uuid::Error),

    /// Error regarding environment variables
    Env(std::env::VarError),

    /// Error when `Url::path_segments_mut()` returns `Err()`
    ///
    /// See
    /// [the docs of Url](https://docs.rs/reqwest/0.10.1/reqwest/struct.Url.html#method.path_segments_mut)
    /// for more information
    UrlCannotBeBase,

    /// General Error when the API key was unable to be created.
    ///
    /// To get more specific errors use the `ApiKey::from_env()`
    /// and `ApiKey::from_home()` functions.
    FailedToLoadApiKey,
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
