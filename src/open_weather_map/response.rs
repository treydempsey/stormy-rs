use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct City {
    base: Option<String>,
    clouds: Option<Clouds>,
    cod: Option<u32>,
    coord: Option<Coord>,
    dt: Option<u64>,
    id: Option<u64>,
    main: Option<Main>,
    name: Option<String>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    sys: Option<Sys>,
    timezone: Option<i32>,
    visibility: Option<u32>,
    wind: Option<Wind>,
    weather: Option<Vec<Condition>>,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    all: Option<u8>,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    icon: Option<String>,
    id: Option<u64>,
    description: Option<String>,
    main: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    lat: Option<f32>,
    lon: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    grnd_level: Option<u32>,
    humidity: Option<u8>,
    pressure: Option<u32>,
    sea_level: Option<u32>,
    temp: Option<f32>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Rain {
    #[serde(alias = "1h")]
    one_hour: Option<f32>,
    #[serde(alias = "3h")]
    three_hour: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Snow {
    #[serde(alias = "1h")]
    one_hour: Option<f32>,
    #[serde(alias = "3h")]
    three_hour: Option<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(alias = "type")]
    type_: Option<u32>,
    id: Option<u64>,
    message: Option<String>,
    country: Option<String>,
    sunrise: Option<u64>,
    sunset: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    deg: Option<u16>,
    speed: Option<f32>,
}
