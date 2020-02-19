#![warn(missing_docs)]

//! # lexoffice
//! The `lexoffice` crate provides a type safe and easy to use client to the
//! [LexOffice API](https://developers.lexoffice.io/docs/)

pub mod client;
pub mod model;
pub mod request;

mod error;
#[cfg(feature = "fs")]
mod fs;
mod mime;
mod result;
mod util;

pub use error::Error;
pub use result::Result;
