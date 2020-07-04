use crate::error::Error;

/// A `Result` where the `Err` is a `lexoffice::Error`
pub type Result<T> = std::result::Result<T, Error>;
