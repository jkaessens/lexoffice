use crate::serde::date;
use crate::types::Date;
use serde::{Deserializer, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
where
    D: Deserializer<'de>,
{
    date::deserialize(deserializer).map(Some)
}

pub fn serialize<S>(
    date: &Option<Date>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = date {
        date::serialize(date, serializer)
    } else {
        serializer.serialize_none()
    }
}
