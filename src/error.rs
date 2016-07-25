extern crate oauth_client as oauth;
extern crate rustc_serialize;

use std::{error, fmt, str};
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    InvalidError(&'static str),  // Something invalid.
    OAuthError(oauth::Error),
    Utf8Error(str::Utf8Error),
    ParserError(json::ParserError),
    DecorderError(json::DecoderError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidError(ref err) => write!(f, "Invalid json error: {}", err),
            Error::OAuthError(ref err) => write!(f, "OAuth error: {}", err),
            Error::Utf8Error(ref err) => write!(f, "UTF8 conversion error: {}", err),
            Error::ParserError(ref err) => write!(f, "JSON parse error: {}", err),
            Error::DecorderError(ref err) => write!(f, "JSON decoding error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidError(ref err) => err,
            Error::OAuthError(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::ParserError(ref err) => err.description(),
            Error::DecorderError(ref err) => err.description(),
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

impl From<json::ParserError> for Error {
    fn from(err: json::ParserError) -> Error {
        Error::ParserError(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::DecorderError(err)
    }
}
