use log::{debug};
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use hyper::Body;
use hyper::client::HttpConnector;
use http::{Request};
use http::Uri;
use http::uri::PathAndQuery;

pub struct Caller{
    pub client: Client<HttpsConnector<HttpConnector>, Body>,
    scheme: &'static str,
    authority: String,
    api_path: &'static str,
}

impl Caller {
    pub fn new(lang: &str) -> Caller {
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Caller{
            client: client,
            scheme: "https",
            authority: format!("{}.wikipedia.org", lang),
            api_path: "/w/api.php",
        }
    }

    pub fn query_params(&self, pageid: u64) -> Request<(Body)> {
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
        let params = serde_urlencoded::to_string(params).unwrap();
        let path_and_query = format!("{}?{}", self.api_path, params);
        let uri = Uri::builder()
            .scheme(self.scheme)
            .authority::<&str>(self.authority.as_ref())
            .path_and_query(path_and_query.parse::<PathAndQuery>().unwrap())
            .build()
            .unwrap();
        debug!(target: "Wikipedia", "URI: {:?}", uri);

        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .unwrap()
    }

    pub fn category_params(&self, cat_name: &str, cont_token: Option<&String>) {
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
        // self.client.get(&self.base_api_url)
        //     .query(&params)
        //     .build()
        //     .unwrap()
    }
}
