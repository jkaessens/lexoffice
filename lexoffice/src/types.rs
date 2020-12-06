//! Types used in the model

use serde::{Deserialize, Serialize};

/// Type for storing date and time
pub type DateTime = chrono::DateTime<chrono::Utc>;
/// Type for storing dates
pub type Date = chrono::NaiveDate;
/// Type for storing a country code
pub type CountryCode = String;

/// Type for safing Currencies
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Currency {
    /// Euro
    EUR,
}
