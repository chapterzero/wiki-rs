use serde::{Deserialize};
use std::fmt;
use std::error::Error;
use reqwest::{Client, Request, StatusCode};

pub struct Wikipedia {
    pub base_api_url: String,
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            base_api_url: format!("https://{}.wikipedia.org/w/api.php", lang),
        }
    }

    pub fn get_page<T: PageFrom>(&self, from: T) -> Result<Page, Box<dyn Error>> {
        let client = Client::new();
        let req = self.create_query_req(&client, &from);
        let mut res = client.execute(req)?;
        if res.status() != StatusCode::OK {
            return Err(GetError{
                from: from.get_params(),
                reason: format!("expected status code 200, got {}", res.status())
            }.into())
        }

        println!("{}", res.text()?);
        Ok(Page{
            id: 1,
            title: "xx".to_string(),
            desc: "xx".to_string(),
        })
    }

    fn create_query_req<T: PageFrom>(&self, client: &Client, from: &T) -> Request {
        let from_param = from.get_params();
        let params: Vec<(&str, &str)> = vec![
            ("action", "query"),
            ("prop", "extracts|categories"),
            ("exintro", "true"),
            ("explaintext", "true"),
            ("format", "json"),
            ("redirects", "1"),
            (from_param.0, &from_param.1),
        ];
        client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct GetError {
    from: (&'static str, String),
    reason: String,
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error when Requesting with param {}: {}, Reason: {}",
           self.from.0,
           self.from.1,
           self.reason,
        )
    }
}

impl Error for GetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Deserialize,Debug)]
pub struct Page {
    pub id: u64,
    pub title: String,
    pub desc: String,
}

pub trait PageFrom: fmt::Debug {
    fn get_params(&self) -> (&'static str, String);
}

impl PageFrom for u64 {
    fn get_params(&self) -> (&'static str, String) {
        ("pageids", self.to_string())
    }
}

impl <'a>PageFrom for &'a str {
    fn get_params(&self) -> (&'static str, String) {
        ("titles", self.to_string())
    }
}
