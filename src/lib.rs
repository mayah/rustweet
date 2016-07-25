extern crate oauth_client as oauth;
extern crate rustc_serialize;

mod error;

use std::collections::HashMap;
use oauth::Token;
use rustc_serialize::json::{self, Json};
use rustc_serialize::Decodable;

pub use error::Error;

const HOME_TIMELINE_URL: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
const SEARCH_TIMELINE_URL: &'static str = "https://api.twitter.com/1.1/search/tweets.json";

pub struct Client {
    consumer_token: oauth::Token<'static>,
    access_token: oauth::Token<'static>,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct User {
    pub name: String,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Tweet {
    pub id_str: String,
    pub created_at: String,
    pub text: String,
    pub user: User,
}

impl Client {
    pub fn new(consumer_key: &str, consumer_secret: &str, access_key: &str, access_secret: &str) -> Client {
        Client {
            consumer_token: Token::new(consumer_key.to_string(), consumer_secret.to_string()),
            access_token: Token::new(access_key.to_string(), access_secret.to_string()),
        }
    }

    pub fn get_timeline(&self) -> Result<Vec<Tweet>, Error> {
        let bytes = try!(oauth::get(HOME_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), None));
        let json_str = try!(std::str::from_utf8(bytes.as_slice()));
        let js = try!(Json::from_str(json_str));
        let mut decoder = json::Decoder::new(js);
        let tweets = try!(Decodable::decode(&mut decoder));

        Ok(tweets)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Tweet>, Error> {
        let mut param = HashMap::new();
        param.insert("q".into(), query.into());

        let bytes = try!(oauth::get(SEARCH_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), Some(&param)));

        let json_str: &str = try!(std::str::from_utf8(bytes.as_slice()));
        let js: Json = try!(Json::from_str(json_str));
        let statuses: Json = match js.find("statuses") {
            Some(x) => x.clone(),
            None => return Err(Error::InvalidError("statuses not found"))
        };
        // let statuses = try!(js.find("statuses")).clone();
        let mut decoder = json::Decoder::new(statuses);
        let d = try!(Decodable::decode(&mut decoder));
        Ok(d)
    }
}
