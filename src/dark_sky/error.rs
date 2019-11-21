use hyper::error::Error as HyperError;
use url::ParseError as UrlParseError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    Error(String),
    HyperError(HyperError),
    JsonError(JsonError),
    UrlParseError(UrlParseError),
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Self {
        Error::HyperError(error)
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Error::JsonError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}

impl From<UrlParseError> for Error {
    fn from(error: UrlParseError) -> Self {
        Error::UrlParseError(error)
    }
}
