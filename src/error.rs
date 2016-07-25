extern crate oauth_client as oauth;
extern crate rustc_serialize;

use std::str;
use rustc_serialize::json;

pub enum Error {
    Invalid(&'static str),  // Something invalid.
    OAuth(oauth::Error),
    Utf8(str::Utf8Error),
    Parser(json::ParserError),
    Decorder(json::DecoderError),
}

impl From<oauth::Error> for Error {
    fn from(err: oauth::Error) -> Error {
        Error::OAuth(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<json::ParserError> for Error {
    fn from(err: json::ParserError) -> Error {
        Error::Parser(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::Decorder(err)
    }
}
