use chrono::{DateTime, Duration, Utc};
use http::uri::{PathAndQuery, Uri};
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{Body, Client as HyperClient, Request};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_tls::HttpsConnector;
use log::debug;
use crate::ProxyConfig;
use crate::request::PageId;
use percent_encoding::{AsciiSet, CONTROLS};
use typed_headers::Credentials;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'/')
    .add(b'?')
    .add(b'`')
    .add(b'\'');

const HYPER_MAX_IDLE: usize = 100;

#[derive(Clone)]
pub struct AsyncCaller {
    pub authority: &'static str,          // mediawiki domain for pageviews
    pub wiki_authority: String,           // wikipedia domain for category & page details
    pub wikidata_authority: &'static str, // wikidata domain for wikidata items Ex: alias
    pub scheme: &'static str,
    pub api_path: &'static str,
    client: Option<HyperClient<HttpsConnector<HttpConnector>, Body>>,
    proxy_client: Option<HyperClient<ProxyConnector<HttpConnector>, Body>>,
}

impl AsyncCaller {
    pub fn new(lang: &str, proxy: Option<ProxyConfig>) -> AsyncCaller {
        let mut client = None;
        let mut proxy_client = None;
        match proxy {
            Some(v) => {
                let connector = HttpConnector::new(4);
                let mut proxy_connector = ProxyConnector::new(connector).unwrap();
                let proxy_uri = v.host.parse().unwrap();
                let mut proxy = Proxy::new(Intercept::All, proxy_uri);
                match v.username {
                    None => (),
                    Some(u) => {
                        proxy.set_authorization(Credentials::basic(u, v.password.unwrap_or("")).unwrap())
                    },
                }
                proxy_connector.add_proxy(proxy);
                proxy_client = Some(
                    HyperClient::builder()
                        .max_idle_per_host(HYPER_MAX_IDLE)
                        .build(proxy_connector),
                );
            }
            None => {
                let https = HttpsConnector::new(4).unwrap();
                client = Some(
                    HyperClient::builder()
                        .max_idle_per_host(HYPER_MAX_IDLE)
                        .build::<_, hyper::Body>(https));
            }
        }

        AsyncCaller {
            client: client,
            proxy_client: proxy_client,
            authority: "wikimedia.org",
            wiki_authority: format!("{}.wikipedia.org", lang),
            wikidata_authority: "www.wikidata.org",
            scheme: "https",
            api_path: "/w/api.php",
        }
    }

    pub fn request(&self, req: Request<Body>) -> ResponseFuture {
        match &self.client {
            Some(c) => c.request(req),
            None => {
                let c = self.proxy_client.as_ref().unwrap();
                c.request(req)
            }
        }
    }

    pub fn query_params<T: PageId>(&self, pageid: &T) -> Request<Body> {
        let q = pageid.to_string();
        let params: Vec<(&str, &str)> = vec![
            ("format", "json"),
            ("action", "query"),
            ("redirects", "1"),
            ("prop", "info|extracts|categories|pageprops"),
            ("ppprop", "wikibase_item"),
            ("exintro", "true"),
            ("explaintext", "true"),
            ("inprop", "url"),
            ("cllimit", "20"),
            ("clshow", "!hidden"),
            (pageid.get_param_name(), &q),
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
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }
}
