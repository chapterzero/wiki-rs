use reqwest::{Client as HttpClient, Request};
use std::fmt;
use super::response::{QueryResponse, Page};
use log::{info};

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
        info!(target: "Wikipedia", "Query Params: {:?}", params);
        self.client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
    }

    pub fn category_params(&self, cat_name: &str, cont_token: Option<&String>) -> Request {
        let mut params: Vec<(&str, &str)> = vec![
            ("format", "json"),
            ("action", "query"),
            ("redirects", "1"),
            ("generator", "categorymembers"),
            ("prop", "info"),
            ("inprop", "url"),
            ("gcmtitle", cat_name),
            ("gcmlimit", "500"),
            ("gcmtype", "page|subcat"),
        ];
        match cont_token {
            Some(token) => {
                params.push(("gcmcontinue", token))
            }
            None => (),
        }
        info!(target: "Wikipedia", "Category Params: {:?}", params);
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
