use reqwest::{Client as HttpClient, Request};
use std::fmt;
use super::response::{QueryResponse, Page};

pub struct Caller{
    pub base_api_url: String,
    pub client: HttpClient,
}

impl Caller {
    pub fn query_params<Q: ToString>(&self, key: &str, q: Q) -> Request {
        let q = q.to_string();
        let params: Vec<(&str, &str)> = vec![
            ("format", "json"),
            ("action", "query"),
            ("redirects", "1"),
            ("prop", "info|extracts|categories"),
            ("exintro", "true"),
            ("explaintext", "true"),
            ("inprop", "url"),
            ("cllimit", "20"),
            ("clshow", "!hidden"),
            (key, &q),
        ];
        self.client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
     
    }

    pub fn create_category(&self, from:&(&str, String)) -> Request {
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
}

pub trait PageFrom: fmt::Debug + ToString + Copy {
    fn extract_page(&self, res: QueryResponse) -> Option<Page>;
}

impl PageFrom for u64 {
    fn extract_page(&self, res: QueryResponse) -> Option<Page> {
        let page = match res.query.pages.get(&self.to_string()) {
            None => return None,
            Some(p) => p,
        };
        Some(page.clone())
    }
}

impl <'a>PageFrom for &'a str {
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
