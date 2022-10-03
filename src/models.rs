use chrono::{prelude::DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::consts::FORMAT_STR_DATETIME_INPUT;
use crate::schemas::input::RawEarthquake;
use crate::utils::date_serializer;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub full: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Earthquake {
    #[serde(with = "date_serializer")]
    pub datetimeutc: DateTime<Utc>,
    pub location: Location,
    pub magnitude: f32,
    pub depth: f32,
}

impl From<RawEarthquake> for Earthquake {
    fn from(raw: RawEarthquake) -> Self {
        Self {
            datetimeutc: Utc
                .datetime_from_str(&raw.datetimeutc, FORMAT_STR_DATETIME_INPUT)
                .unwrap(),
            location: Location {
                full: raw.location.value,
                state: raw.location.options.sort_value,
            },
            magnitude: raw.magnitude.value,
            depth: raw.depth.value.parse::<f32>().unwrap_or(0_f32),
        }
    }
}
