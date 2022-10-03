use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInfo {
    pub sort_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationInfo {
    pub options: OptionsInfo,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepthInfo {
    options: OptionsInfo,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagnitudeOptionStyle {
    color: String,
    #[serde(alias = "font-weight")]
    font_weight: String,
    #[serde(alias = "font-size")]
    font_size: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagnitudeOptions {
    style: MagnitudeOptionStyle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagnitudeInfo {
    options: MagnitudeOptions,
    pub value: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawEarthquake {
    id: u32,
    datetime: String,
    pub location: LocationInfo,
    pub latitude: f32,
    pub longitude: f32,
    pub depth: DepthInfo,
    pub magnitude: MagnitudeInfo,
    pub datetimeutc: String,
}
