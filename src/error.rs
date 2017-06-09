use std::{error, fmt, str};

use oauth;
use serde_json;

#[derive(Debug)]
pub enum Error {
    InvalidError(&'static str),  // Something invalid.
    OAuthError(oauth::Error),
    Utf8Error(str::Utf8Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidError(ref err) => write!(f, "Invalid json error: {}", err),
            Error::OAuthError(ref err) => write!(f, "OAuth error: {}", err),
            Error::Utf8Error(ref err) => write!(f, "UTF8 conversion error: {}", err),
            Error::JsonError(ref err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidError(ref err) => err,
            Error::OAuthError(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::JsonError(ref err) => err.description(),
        }
    }
}

impl From<oauth::Error> for Error {
    fn from(err: oauth::Error) -> Error {
        Error::OAuthError(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}
