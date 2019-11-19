use hyper::error::Error as HyperError;

#[derive(Debug)]
pub enum Error {
    Error(String),
    HyperError(HyperError),
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Self {
        Error::HyperError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Error(error)
    }
}
