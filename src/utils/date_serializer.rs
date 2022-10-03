use crate::consts::{FORMAT_STR_DATETIME_INPUT, FORMAT_STR_DATETIME_OUTPUT};
use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT_STR_DATETIME_OUTPUT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, FORMAT_STR_DATETIME_INPUT)
        .map_err(serde::de::Error::custom)
}
