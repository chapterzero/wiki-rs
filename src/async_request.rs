use hyper::{Client as HyperClient, Body, Request};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use chrono::{Utc, DateTime, Duration};
use percent_encoding:: {AsciiSet, CONTROLS};
use http::uri::{Uri, PathAndQuery};
use log::debug;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'/').add(b'?').add(b'`').add(b'\'');

pub struct AsyncCaller {
    pub client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    pub authority: &'static str,    // mediawiki domain
    pub wiki_authority: String,     // wikipedia domain
    pub scheme: &'static str,
}

impl AsyncCaller {
    pub fn new(lang: &str) -> AsyncCaller {
        let https = HttpsConnector::new(4).unwrap();
        AsyncCaller {
            authority: "wikimedia.org",
            scheme: "https",
            wiki_authority: format!("{}.wikipedia.org", lang),
            client: HyperClient::builder().build::<_, hyper::Body>(https),
        }
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
