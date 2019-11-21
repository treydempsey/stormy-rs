use futures::Future;
use futures::future::{Either, err, ok};
use futures::stream::Stream;

use hyper::Client;
use hyper::Uri;
use hyper_tls::HttpsConnector;
use url::Url;

use super::error::Error;
use super::response::ForecastResponse;

pub fn forecast(key: &str, latitude: f32, longitude: f32) -> impl Future<Item = ForecastResponse, Error = Error> {
    match build_url(key, latitude, longitude) {
        Ok(url) => {
            println!("URL: {:?}", url);

            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder().build::<_, hyper::Body>(https);

            let client_fut = client
                .get(url)
                .and_then(|res| {
                    println!("Hyper response: {:?}", res.status());
                    Ok(res)
                })
                .and_then(|res| {
                    res.into_body().fold(Vec::new(), |mut acc, chunk| {
                        acc.extend_from_slice(&*chunk);
                        ok::<_, hyper::Error>(acc)
                    })
                })
                .and_then(|v| Ok(String::from_utf8_lossy(&v).into_owned()))
                .from_err::<Error>()
                .and_then(|json| { println!("json: {}", json); Ok(serde_json::from_str::<ForecastResponse>(&json)?) })
                .from_err::<Error>();

            Either::A(client_fut)
        }
        Err(e) => {
            println!("Error parsing: {:?}", e);
            Either::B(err(Error::Error("Error parsing API response".to_owned())))
        }
    }
}

fn build_url(key: &str, latitude: f32, longitude: f32) -> Result<Uri, Error> {
    match Url::parse_with_params(format!("https://api.darksky.net/forecast/{}/{},{}", key, latitude, longitude).as_ref(), &[("exclude", "minutely,hourly,daily")]) {
        Ok(url) => Ok(url.to_string().parse::<Uri>().unwrap()),
        Err(e) => Err(Error::UrlParseError(e)),
    }
}
