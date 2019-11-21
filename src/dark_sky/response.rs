use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::serde_utils::{ts_seconds, opt_ts_seconds};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Forecast {
    ForecastResponse(ForecastResponse),
    ApiError(ApiError),
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ForecastResponse {
    pub latitude: f32,
    pub longitude: f32,
    pub timezone: String,
    pub offset: Option<i8>,
    pub currently: Option<DataPoint>,
    pub minutely: Option<DataBlock>,
    pub hourly: Option<DataBlock>,
    pub daily: Option<DataBlock>,
    pub alerts: Option<Vec<Alert>>,
    pub flags: Option<Flag>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: u64,
    pub error: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    pub apparent_temperature: Option<f32>,
    pub apparent_temperature_high: Option<f32>,
    pub apparent_temperature_high_time: Option<i64>,
    pub apparent_temperature_low: Option<f32>,
    pub apparent_temperature_low_time: Option<i64>,
    pub apparent_temperature_max: Option<f32>,
    pub apparent_temperature_max_time: Option<i64>,
    pub apparent_temperature_min: Option<f32>,
    pub apparent_temperature_min_time: Option<i64>,
    pub cloud_cover: Option<f32>,
    pub dew_point: Option<f32>,
    pub humidity: Option<f32>,
    pub icon: Option<Icon>,
    pub moon_phase: Option<f32>,
    pub nearest_storm_bearing: Option<u16>,
    pub nearest_storm_distance: Option<i32>,
    pub ozone: Option<f32>,
    pub precip_accumulation: Option<f32>,
    pub precip_intensity: Option<f32>,
    pub precip_intensity_error: Option<f32>,
    pub precip_intensity_max: Option<f32>,
    pub precip_intensity_max_time: Option<i64>,
    pub precip_probability: Option<f32>,
    pub precip_type: Option<String>,
    pub pressure: Option<f32>,
    pub summary: Option<String>,
    pub sunrise_time: Option<i64>,
    pub sunset_time: Option<i64>,
    pub temperature: Option<f32>,
    pub temperature_high: Option<f32>,
    pub temperature_high_time: Option<i64>,
    pub temperature_low: Option<f32>,
    pub temperature_low_time: Option<i64>,
    pub temperature_max: Option<f32>,
    pub temperature_max_time: Option<i64>,
    pub temperature_min: Option<f32>,
    pub temperature_min_time: Option<i64>,
    #[serde(deserialize_with = "ts_seconds")]
    pub time: NaiveDateTime,
    pub uv_index: Option<u8>,
    pub uv_index_time: Option<i64>,
    pub visibility: Option<f32>,
    pub wind_bearing: Option<u16>,
    pub wind_gust: Option<f32>,
    #[serde(with = "opt_ts_seconds")]
    pub wind_gust_time: Option<NaiveDateTime>,
    pub wind_speed: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DataBlock {
    pub data: Vec<DataPoint>,
    pub summary: Option<String>,
    pub icon: Option<Icon>,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Alert {
    pub description: String,
    #[serde(deserialize_with = "ts_seconds")]
    pub expires: NaiveDateTime,
    pub regions: Vec<String>,
    pub severity: Severity,
    #[serde(deserialize_with = "ts_seconds")]
    pub time: NaiveDateTime,
    pub title: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Flag {
    #[serde(rename = "darksky-unavailable")]
    pub darksky_unavailable: Option<bool>,
    #[serde(rename = "nearest-station")]
    pub nearest_station: Option<f32>,
    pub sources: Vec<Source>,
    pub units: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum Icon {
    ClearDay,
    ClearNight,
    Cloudy,
    Fog,
    Hail,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Rain,
    Sleet,
    Snow,
    Thunderstorm,
    Tornado,
    Wind,
    Unknown(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum Severity {
    Advisory,
    Watch,
    Warning,
    Unknown(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum Source {
    Cmc,
    Darksky,
    Ecpa,
    Gfs,
    Hrrr,
    Icon,
    Imo,
    Isd,
    Madis,
    Meteoalarm,
    Nam,
    NearestPrecip,
    Nwspa,
    Sref,
    Unknown(String),
}

impl Default for ForecastResponse {
    fn default() -> Self {
        ForecastResponse {
            latitude: f32::default(),
            longitude: f32::default(),
            timezone: String::default(),
            offset: None,
            currently: None,
            minutely: None,
            hourly: None,
            daily: None,
            alerts: None,
            flags: None,
        }
    }
}

impl Default for DataPoint {
    fn default() -> Self {
        DataPoint {
            apparent_temperature: None,
            apparent_temperature_high: None,
            apparent_temperature_high_time: None,
            apparent_temperature_low: None,
            apparent_temperature_low_time: None,
            apparent_temperature_max: None,
            apparent_temperature_max_time: None,
            apparent_temperature_min: None,
            apparent_temperature_min_time: None,
            cloud_cover: None,
            dew_point: None,
            humidity: None,
            icon: None,
            moon_phase: None,
            nearest_storm_bearing: None,
            nearest_storm_distance: None,
            ozone: None,
            precip_accumulation: None,
            precip_intensity: None,
            precip_intensity_error: None,
            precip_intensity_max: None,
            precip_intensity_max_time: None,
            precip_probability: None,
            precip_type: None,
            pressure: None,
            summary: None,
            sunrise_time: None,
            sunset_time: None,
            temperature: None,
            temperature_high: None,
            temperature_high_time: None,
            temperature_low: None,
            temperature_low_time: None,
            temperature_max: None,
            temperature_max_time: None,
            temperature_min: None,
            temperature_min_time: None,
            time: chrono::naive::MIN_DATE.and_hms(0, 0, 0),
            uv_index: None,
            uv_index_time: None,
            visibility: None,
            wind_bearing: None,
            wind_gust: None,
            wind_gust_time: None,
            wind_speed: None,
        }
    }
}

impl Default for DataBlock {
    fn default() -> Self {
        DataBlock {
            data: Vec::new(),
            summary: None,
            icon: None,
        }
    }
}

impl Default for Alert {
    fn default() -> Self {
        Alert {
            description: String::default(),
            expires: chrono::naive::MIN_DATE.and_hms(0, 0, 0),
            regions: Vec::new(),
            severity: Severity::Unknown(String::default()),
            time: chrono::naive::MIN_DATE.and_hms(0, 0, 0),
            title: String::default(),
            uri: String::default(),
        }
    }
}

impl Default for Flag {
    fn default() -> Self {
        Flag {
            darksky_unavailable: None,
            nearest_station: None,
            sources: Vec::new(),
            units: String::default(),
        }
    }
}
