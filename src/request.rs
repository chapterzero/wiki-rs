use reqwest::{Client as ReqwestClient, Request};
use hyper::{Client as HyperClient, Body};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use log::{debug};
use chrono::{Utc, DateTime, Duration};
use percent_encoding:: {AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'/').add(b'?').add(b'`').add(b'\'');

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
        debug!(target: "Wikipedia", "Category Params: {:?}", params);
        self.reqwest_client.get(&self.base_api_url)
            .query(&params)
            .build()
            .unwrap()
    }

}

pub struct AsyncCaller {
    pub client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub authority: String,
}

impl AsyncCaller {
    pub fn new(lang: &str) -> AsyncCaller {
        let https = HttpsConnector::new(4).unwrap();
        AsyncCaller {
            authority: format!("{}.wikipedia.org", lang),
            client: HyperClient::builder().build::<_, hyper::Body>(https),
        }
    }

    pub fn get_pageviews_url(&self, page_title: &str, month_retention: i64) -> String {
        let now: DateTime<Utc> = Utc::now();
        let month_ago: DateTime<Utc> = now - Duration::days(month_retention * 30);
        let api_url = format!(
            "https://wikimedia.org/api/rest_v1/metrics/pageviews/per-article/{}/all-access/all-agents/{}/monthly/{}/{}",
            self.authority,
            percent_encoding::utf8_percent_encode(page_title, FRAGMENT).collect::<String>(),
            month_ago.format("%Y%m01"),
            now.format("%Y%m01"),
        );
        api_url
    }
}
