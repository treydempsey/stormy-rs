use std::fmt;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CityResponse {
    City(City),
    ApiError(ApiError),
}

#[derive(Deserialize, Debug)]
pub struct City {
    pub base: Option<String>,
    pub clouds: Option<Clouds>,
    pub cod: Option<u32>,
    pub coord: Option<Coord>,
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    pub id: Option<u64>,
    pub main: Option<Main>,
    pub name: Option<String>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub sys: Option<Sys>,
    pub timezone: Option<i32>,
    pub visibility: Option<u32>,
    pub wind: Option<Wind>,
    pub weather: Option<Vec<Condition>>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub cod: Option<String>,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub icon: Option<String>,
    pub id: Option<u64>,
    pub description: Option<String>,
    pub main: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lat: Option<f32>,
    pub lon: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub grnd_level: Option<u32>,
    pub humidity: Option<u8>,
    pub pressure: Option<u32>,
    pub sea_level: Option<u32>,
    pub temp: Option<f32>,
    pub temp_max: Option<f32>,
    pub temp_min: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Rain {
    #[serde(alias = "1h")]
    pub one_hour: Option<f32>,
    #[serde(alias = "3h")]
    pub three_hour: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Snow {
    #[serde(alias = "1h")]
    pub one_hour: Option<f32>,
    #[serde(alias = "3h")]
    pub three_hour: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(alias = "type")]
    pub type_: Option<u32>,
    pub id: Option<u64>,
    pub message: Option<String>,
    pub country: Option<String>,
    #[serde(with = "ts_seconds")]
    pub sunrise: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub sunset: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub deg: Option<f32>,
    pub speed: Option<f32>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let no_code = String::from("0");
        let cod = self.cod.as_ref().unwrap_or(&no_code);
        let no_message = String::from("<no message>");
        let message = self.message.as_ref().unwrap_or(&no_message);
        write!(f, "{}: {}", cod, message)
    }
}
