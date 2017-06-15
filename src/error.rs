use std::ffi::NulError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    ProxyResolveError,
    InvalidUrl(NulError),
    NonUtf8Proxy(FromUtf8Error),
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error::InvalidUrl(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::NonUtf8Proxy(err)
    }
}
