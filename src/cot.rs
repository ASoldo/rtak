use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "event")]
pub struct CoT {
    #[serde(rename = "type", default)]
    pub cot_type: String,

    #[serde(default)]
    pub uid: String,

    #[serde(default)]
    pub time: String,

    #[serde(default)]
    pub stale: String,

    #[serde(default)]
    pub how: String,

    #[serde(default)]
    pub point: Option<Point>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    #[serde(rename = "lat", default)]
    pub lat: f64,
    #[serde(rename = "lon", default)]
    pub lon: f64,
    #[serde(rename = "hae", default)]
    pub hae: f64,
    #[serde(rename = "ce", default)]
    pub ce: f64,
    #[serde(rename = "le", default)]
    pub le: f64,
}
