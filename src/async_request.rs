use hyper::{Client as HyperClient, Body};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use chrono::{Utc, DateTime, Duration};
use percent_encoding:: {AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'/').add(b'?').add(b'`').add(b'\'');


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
