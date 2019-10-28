use reqwest::{Client as ReqwestClient, Request};
use super::response::{QueryResponse, Page};
use log::{debug};

pub struct Caller{
    pub base_api_url: String,
    pub reqwest_client: ReqwestClient,
}

impl Caller {
    pub fn new(lang: &str) -> Caller {
        let authority = format!("{}.wikipedia.org", lang);
        Caller {
            base_api_url: format!("https://{}/w/api.php", &authority),
            reqwest_client: ReqwestClient::new(),
        }
    }

    pub fn query_params_sync<T: PageId>(&self, pageid: &T) -> Request {
        let q = pageid.to_string();
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
            (pageid.get_param_name(), &q),
        ];
        debug!(target: "Wikipedia", "Query Params: {:?}", params);
        self.reqwest_client.get(&self.base_api_url)
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
            ("prop", "info|pageprops"),
            ("ppprop", "wikibase_item"),
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
        debug!(target: "Wikipedia", "Category Params: {:?}", params);
        self.reqwest_client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
    }
}

// PageId could be u64 / &str, they only differ in parameter name and
// how to extract page from result
// NOTE when using &str, there is no guarantee page returned is correct
// because in response, page is indexed with pageid
pub trait PageId: std::fmt::Display + std::fmt::Debug {
    fn get_param_name(&self) -> &'static str;
    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page>;
}

impl PageId for u64 {
    fn get_param_name(&self) -> &'static str {
        "pageids"
    }

    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page> {
        resp.query.pages.get(&self.to_string()).map(|p| p.clone())
    }
}

impl PageId for &str {
    fn get_param_name(&self) -> &'static str {
        "titles"
    }

    fn get_page_from_response(&self, resp: &QueryResponse) -> Option<Page> {
        resp.query.pages.values().next().map(|p| p.clone())
    }
}
