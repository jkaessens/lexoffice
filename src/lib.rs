pub mod client;
pub mod error;
pub mod model;
pub mod request;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
