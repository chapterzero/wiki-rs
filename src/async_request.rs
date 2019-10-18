use hyper::{Client as HyperClient, Body, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use chrono::{Utc, DateTime, Duration};
use percent_encoding:: {AsciiSet, CONTROLS};
use http::uri::{Uri, PathAndQuery};
use log::debug;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'/').add(b'?').add(b'`').add(b'\'');

#[derive(Clone)]
pub struct AsyncCaller {
    pub client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub authority: &'static str,            // mediawiki domain for pageviews
    pub wiki_authority: String,             // wikipedia domain for category & page details
    pub wikidata_authority: &'static str,   // wikidata domain for wikidata items Ex: alias
    pub scheme: &'static str,
    pub api_path: &'static str,
}

impl AsyncCaller {
    pub fn new(lang: &str) -> AsyncCaller {
        let https = HttpsConnector::new(4).unwrap();
        AsyncCaller {
            client: HyperClient::builder().build::<_, hyper::Body>(https),
            authority: "wikimedia.org",
            wiki_authority: format!("{}.wikipedia.org", lang),
            wikidata_authority: "www.wikidata.org",
            scheme: "https",
            api_path: "/w/api.php",
        }
    }

    pub fn query_params(&self, pageid: u64) -> Request<Body> {
        let pageid = pageid.to_string();
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
            ("pageids", &pageid),
        ];
        Request::builder()
            .uri(self.build_wiki_uri(&self.wiki_authority, &params))
            .body(Body::empty())
            .unwrap()
    }

    pub fn wikidata_params(&self, wikidata_id: &str, lang: &str) -> Request<Body> {
        let params: Vec<(&str, &str)> = vec![
            ("format", "json"),
            ("action", "wbgetentities"),
            ("ids", wikidata_id),
            ("props", "aliases"),
            ("languages", lang),
        ];
        Request::builder()
            .uri(self.build_wiki_uri(self.wikidata_authority, &params))
            .body(Body::empty())
            .unwrap()
    }

    fn build_wiki_uri(&self, authority: &str, params: &[(&str, &str)]) -> Uri {
        let params = serde_urlencoded::to_string(params).unwrap();
        let path_and_query = format!("{}?{}", self.api_path, params);
        let uri = Uri::builder()
            .scheme(self.scheme)
            .authority(authority)
            .path_and_query(path_and_query.parse::<PathAndQuery>().unwrap())
            .build()
            .unwrap();
        debug!(target: "Wikipedia", "URI: {:?}", uri);
        uri
    }

    pub fn get_pageviews_req(&self, page_title: &str, month_retention: i64) -> Request<Body> {
        let now: DateTime<Utc> = Utc::now();
        let month_ago: DateTime<Utc> = now - Duration::days(month_retention * 30);
        let path = format!(
            "/api/rest_v1/metrics/pageviews/per-article/{}/all-access/all-agents/{}/monthly/{}/{}",
            self.wiki_authority,
            percent_encoding::utf8_percent_encode(page_title, FRAGMENT).collect::<String>(),
            month_ago.format("%Y%m01"),
            now.format("%Y%m01"),
        );
        let uri = Uri::builder()
            .scheme(self.scheme)
            .authority::<&str>(self.authority.as_ref())
            .path_and_query(path.parse::<PathAndQuery>().unwrap())
            .build()
            .unwrap();
        debug!(target: "Wikipedia", "URI: {:?}", uri);
        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }
}
