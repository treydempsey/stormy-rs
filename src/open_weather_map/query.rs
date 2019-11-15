use futures::Future;
use futures::future::ok;
use futures::stream::Stream;

use hyper::Client;
use hyper::Uri;

pub fn city(message: String) -> impl Future<Item = String, Error = ()> {
    let city = message.trim_start_matches("pickles").trim().to_owned();
    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, "8aa923241cb3621dcb8ffb6f9b45e0c8").parse::<Uri>().unwrap();
    Client::new()
        .get(url)
        .and_then(|res| {
            res.into_body().fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                ok::<_, hyper::Error>(acc)
            })
        })
        .and_then(|v| Ok(String::from_utf8_lossy(&v).into_owned()))
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}
