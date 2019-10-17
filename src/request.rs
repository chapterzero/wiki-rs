use reqwest::{Client as ReqwestClient, Request};
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

    pub fn query_params_sync(&self, pageid: u64) -> Request {
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
            ("pageids", &q),
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
