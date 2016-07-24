extern crate oauth_client as oauth;
extern crate rustc_serialize;

use std::collections::HashMap;
use oauth::Token;
use rustc_serialize::json::{self, Json};
use rustc_serialize::Decodable;

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

    pub fn get_timeline(&self) -> Result<Vec<Tweet>, oauth::Error> {
        let bytes = try!(oauth::get(HOME_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), None));
        let json_str = std::str::from_utf8(bytes.as_slice()).unwrap();

        let js = Json::from_str(json_str).unwrap();
        let mut decoder = json::Decoder::new(js);
        let tweets = Decodable::decode(&mut decoder).unwrap();

        Ok(tweets)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Tweet>, oauth::Error> {
        let mut param = HashMap::new();
        param.insert("q".into(), query.into());

        let bytes = match oauth::get(SEARCH_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), Some(&param)) {
            Ok(x) => x,
            Err(oauth::Error::HttpStatus(resp)) => {
                let s = std::str::from_utf8(resp.get_body()).unwrap();
                println!("body={}", s);

                assert!(false);
                Vec::new()
            },
            Err(x) => {
                return Err(x)
            }
        };

        let statuses = {
            let json_str: &str = std::str::from_utf8(bytes.as_slice()).unwrap();
            let js: Json = Json::from_str(json_str).unwrap();
            js.find("statuses").unwrap().clone()
        };
        let mut decoder = json::Decoder::new(statuses);
        let tweets: Vec<Tweet> = Decodable::decode(&mut decoder).unwrap();
        Ok(tweets)
    }
}
