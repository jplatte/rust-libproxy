use std::ffi::NulError;
use std::string::FromUtf8Error;

/// Couldn't resolve the proxy for the given URL.
#[derive(Debug)]
pub struct ProxyResolutionError;

#[derive(Debug)]
pub enum Error {
    /// Couldn't resolve the proxy for the given URL.
    ProxyResolutionError(ProxyResolutionError),

    /// The provided URL couldn't be converted to a CString.
    InvalidUrl(NulError),

    /// One of the returned proxy URLs was not valid UTF-8.
    NonUtf8Proxy(FromUtf8Error),
}

impl From<ProxyResolutionError> for Error {
    fn from(err: ProxyResolutionError) -> Error {
        Error::ProxyResolutionError(err)
    }
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
