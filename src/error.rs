extern crate oauth_client as oauth;
extern crate rustc_serialize;

use std::str;
use rustc_serialize::json;

pub enum Error {
    InvalidError(&'static str),  // Something invalid.
    OAuthError(oauth::Error),
    Utf8Error(str::Utf8Error),
    ParserError(json::ParserError),
    DecorderError(json::DecoderError),
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
