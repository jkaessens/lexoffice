//! Error handling for the `lexoffice` crate
use derive_error::Error;
use reqwest::StatusCode;
use serde::Deserialize;
use std::fmt;

/// This enum represents error messages from LexOffice
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Message {
    /// error messages from LexOffice
    #[serde(rename = "message")]
    Message(String),
    /// legacy error messages from LexOffice
    #[serde(rename = "IssueList")]
    IssueList(Vec<Issue>),
}

impl fmt::Display for Message {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Message(x) => x.fmt(f)?,
            Self::IssueList(x) => {
                let mut first = true;
                for item in x {
                    if !first {
                        write!(f, ", ")?;
                        first = false;
                    }
                    item.fmt(f)?;
                }
            }
        }
        Ok(())
    }
}

/// Represents an issue item, used in the legacy error message handling of Lexoffice
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Issue {
    i18n_key: String,
    source: Option<String>,
    _type: String,
    additional_data: Option<String>,
    args: Option<String>,
}

impl fmt::Display for Issue {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self._type, self.i18n_key)?;
        if let Some(additional_data) = &self.additional_data {
            write!(f, " data: {}", additional_data)?;
        }
        if let Some(args) = &self.args {
            write!(f, " args({})", args)?;
        }
        Ok(())
    }
}

/// The Errors that may occur working with this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// Errors from the exOffice API
    ///
    /// See
    /// [The official API](https://developers.lexoffice.io/docs/#error-codes)
    /// for more information.
    LexOffice(LexOfficeError),

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

    /// When converting an object to its uuid
    NoUuid,
}

impl Unpin for Error {}

/// An error returned from LexOffice
#[derive(Debug)]
pub struct LexOfficeError {
    status: StatusCode,
    message: Message,
}

impl LexOfficeError {
    pub(crate) fn new(status: StatusCode, message: Message) -> Self {
        Self { status, message }
    }
}

impl fmt::Display for LexOfficeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "HTTP Error {}: {}", self.status, self.message)
    }
}

impl std::error::Error for LexOfficeError {}
