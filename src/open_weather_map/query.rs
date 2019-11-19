use futures::Future;
use futures::future::{Either, err, ok};
use futures::stream::Stream;

use crate::open_weather_map::error::Error;
use hyper::Client;
use url::Url;

pub fn city(appid: &str, message: String) -> impl Future<Item = String, Error = Error> {
    let city = message.trim_start_matches("pickles").trim();
    
    match Url::parse_with_params("http://api.openweathermap.org/data/2.5/weather", &[("q", city), ("appid", appid)]) {
        Ok(api_url) => {
            let hyper_url = api_url.to_string().parse::<hyper::Uri>().unwrap();
            let client_fut = Client::new()
                .get(hyper_url)
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
                .map_err(|err| {
                    eprintln!("Hyper error: {}", err);
                    Error::HyperError(err)
                });

            Either::A(client_fut)
        }
        Err(e) => {
            println!("Error parsing: {}", e);
            Either::B(err(Error::Error("Error parsing API response".to_owned())))
        }
    }
}
