pub mod response;

use std::fmt;
use std::error::Error;
use reqwest::{Client as HttpClient, Request, StatusCode};
use response::{QueryResponse, Page};

pub struct Wikipedia {
    pub base_api_url: String,
    client: HttpClient,
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            base_api_url: format!("https://{}.wikipedia.org/w/api.php", lang),
            client: HttpClient::new(),
        }
    }

    pub fn get_page<T: PageFrom>(&self, from: T) -> Result<Page, Box<dyn Error>> {
        let from_param = from.get_params();
        let req = self.create_query_req(&from_param);
        let mut res = self.client.execute(req)?;
        if res.status() != StatusCode::OK {
            return Err(GetError{
                from: from_param,
                reason: format!("expected status code 200, got {}", res.status())
            }.into())
        }

        let q: QueryResponse = res.json()?;
        let page = match from.extract_page(q) {
            Some(p) => p,
            None => {
                return Err(GetError{
                    from: from_param,
                    reason: "Unable to get page from result, either empty or wrong page id in pages".to_string(),
                }.into())
            }
        };

        return Ok(page)
    }

    fn create_query_req(&self, from: &(&str, String)) -> Request {
        let params: Vec<(&str, &str)> = vec![
            ("action", "query"),
            ("prop", "extracts|categories"),
            ("exintro", "true"),
            ("explaintext", "true"),
            ("format", "json"),
            ("redirects", "1"),
            ("cllimit", "20"),
            ("clshow", "!hidden"),
            (from.0, &from.1),
        ];
        self.client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
    }


    fn some(&self, res: QueryResponse) -> String {
        res.batchcomplete.unwrap()
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
        None
    }
}

pub trait PageFrom: fmt::Debug {
    fn get_params(&self) -> (&'static str, String);
    fn extract_page(&self, res: QueryResponse) -> Option<Page>;
}

impl PageFrom for u64 {
    fn get_params(&self) -> (&'static str, String) {
        ("pageids", self.to_string())
    }

    fn extract_page(&self, res: QueryResponse) -> Option<Page> {
        let page = match res.query.pages.get(&self.to_string()) {
            None => return None,
            Some(p) => p,
        };
        Some(page.clone())
    }
}

impl <'a>PageFrom for &'a str {
    fn get_params(&self) -> (&'static str, String) {
        ("titles", self.to_string())
    }

    fn extract_page(&self, res: QueryResponse) -> Option<Page> {
        for key in &res.query.pages {
            if key.0 == "-1" {
                continue
            }
            return Some(res.query.pages.get(key.0).unwrap().clone())
        }
        None
    }
}
