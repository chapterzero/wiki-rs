#[macro_use]
extern crate lazy_static;

pub mod r#async;
mod async_request;
pub mod errors;
mod request;
pub mod response;

use regex::Regex;

pub struct ProxyConfig<'a> {
    pub host: &'a str,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

pub enum Lang {
    ID,
    EN,
}

impl Lang {
    pub fn get_wiki_authority(&self) -> &'static str {
        match self {
            Self::ID => "id.wikipedia.org",
            Self::EN => "en.wikipedia.org",
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::ID => "id",
            Self::EN => "en",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "id" => Some(Self::ID),
            "en" => Some(Self::EN),
            _ => None,
        }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"^https?://([a-z]{2})\.wikipedia\.org\.*"#).unwrap();
        }
        RE.captures(url).and_then(|capture| {
            capture
                .get(1)
                .and_then(|lang| Self::from_str(lang.as_str()))
        })
    }
}
