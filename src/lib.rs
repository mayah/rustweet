extern crate oauth_client as oauth;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

mod error;

use std::collections::HashMap;
use oauth::Token;

pub use error::Error;

const HOME_TIMELINE_URL: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
const SEARCH_TIMELINE_URL: &'static str = "https://api.twitter.com/1.1/search/tweets.json";

pub struct Client {
    consumer_token: oauth::Token<'static>,
    access_token: oauth::Token<'static>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
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
        let bytes = oauth::get(HOME_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), None)?;
        let json_str = std::str::from_utf8(bytes.as_slice())?;
        let tweets: Vec<Tweet> = serde_json::from_str(json_str)?;
        Ok(tweets)
    }

    pub fn search(&self, query: &str) -> Result<Vec<Tweet>, Error> {
        let mut param = HashMap::new();
        param.insert("q".into(), query.into());

        let bytes = oauth::get(SEARCH_TIMELINE_URL, &self.consumer_token, Some(&self.access_token), Some(&param))?;
        let json_str: &str = std::str::from_utf8(bytes.as_slice())?;
        let tweets: Vec<Tweet> = serde_json::from_str(json_str)?;
        Ok(tweets)
    }
}
