use crate::types::Date;
use serde::{de, Deserializer, Serializer};
use std::fmt;

struct DateVisitor;

impl<'de> de::Visitor<'de> for DateVisitor {
    type Value = Date;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a date in yyyy-MM-dd format")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Date::parse_from_str(v, "%Y-%m-%d")
            .map_err(|_| de::Error::custom("Invalid date"))
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DateVisitor)
}

pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}
