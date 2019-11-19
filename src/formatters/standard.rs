use crate::open_weather_map::response::{City, Clouds, Wind};

use chrono::FixedOffset;

pub fn format(city: &City) -> String {
    let mut output: Vec<String> = Vec::new();

    let tzoffset = FixedOffset::east(city.timezone.unwrap_or(0));

    match &city.name {
        Some(name) => {
            let mut s = String::with_capacity(20);
            s.push_str("["); s.push_str(&name); s.push_str("]");
            output.push(s);
        },
        None => (),
    }

    {
        let mut s = String::with_capacity(30);
        let dt = city.dt + tzoffset;
        s.push_str("("); s.push_str(&dt.format("%a %b %e %T %Y").to_string()); s.push_str(")");
        output.push(s);
    }

    match &city.weather {
        Some(weather) if weather.len() > 0 && weather[0].description.is_some() => {
            let mut s = String::with_capacity(20);
            s.push_str(&weather[0].description.as_ref().unwrap());
            output.push(s);
        }
        _ => (),
    }

    match &city.main {
        Some(main) => {
            match &main.temp {
                Some(temp) => output.push(format!("{:.1}°F ({:.1}°C)", k_to_f(temp), k_to_c(temp))),
                None => (),
            }

            match &main.temp_max {
                Some(temp_max) => output.push(format!("High: {:.1}°F ({:.1}°C)", k_to_f(temp_max), k_to_c(temp_max))),
                None => (),
            }

            match &main.temp_min {
                Some(temp_min) => output.push(format!("Low: {:.1}°F ({:.1}°C)", k_to_f(temp_min), k_to_c(temp_min))),
                None => (),
            }

            match &main.humidity {
                Some(humidity) => output.push(format!("Humidity: {}%", humidity)),
                None => (),
            }

            match &main.pressure {
                Some(pressure) => output.push(format!("Pressure: {} hPa", pressure)),
                None => (),
            }
        }
        None => (),
    }

    match &city.clouds {
        Some(Clouds { all: Some(pct) }) => output.push(format!("Cloudiness: {}%", pct)),
        _ => (),
    }

    match &city.wind {
        Some(Wind { deg: Some(deg), speed: Some(speed) }) => output.push(format!("Wind: {:.1} MPH ({})", speed, deg_to_direction(deg))),
        _ => (),
    }

    match &city.sys {
        Some(sys) => {
					  let sunrise = sys.sunrise + tzoffset;
					  let sunset = sys.sunset + tzoffset;
            output.push(format!("Sunrise: {} UTC{:+}", sunrise.format("%I:%M:%S %p"), tzoffset.local_minus_utc() / 3600));
            output.push(format!("Sunset: {} UTC{:+}", sunset.format("%I:%M:%S %p"), tzoffset.local_minus_utc() / 3600));
        }
        None => (),
    }

    output.join(" ")
}

fn k_to_f(k: &f32) -> f32 {
    k * 9.0 / 5.0 - 459.67
}

fn k_to_c(k: &f32) -> f32 {
    k - 273.15
}

fn deg_to_direction(deg: &f32) -> &str {
    if (*deg >= 337.5 && *deg <= 360.0) || (*deg >= 0.0 && *deg < 22.5) {
        "E"
    }
    else if *deg >= 22.5 && *deg < 67.5 {
        "NE"
    }
    else if *deg >= 67.5 && *deg < 112.5 {
        "N"
    }
    else if *deg >= 112.5 && *deg < 157.5 {
        "NW"
    }
    else if *deg >= 157.5 && *deg < 202.5 {
        "W"
    }
    else if *deg >= 202.5 && *deg < 247.5 {
        "SW"
    }
    else if *deg >= 247.5 && *deg < 292.5 {
        "S"
    }
    else if *deg >= 292.5 && *deg < 337.5 {
        "SE"
    }
    else {
        ""
    }
}
