use crate::async_request::AsyncCaller;
use crate::errors::*;
use crate::response::Page;
use crate::response::*;
use std::collections::HashSet;
use futures::future::Future;
use futures::stream::Stream;
use log::{warn};

#[derive(Clone)]
pub struct WikipediaAsync {
    caller: AsyncCaller,
}

impl WikipediaAsync {
    pub fn new(lang: &str, proxy: Option<&Vec<String>>) -> WikipediaAsync {
        WikipediaAsync {
            caller: AsyncCaller::new(lang, proxy),
        }
    }

    pub fn get_page(&self, pageid: u64) -> impl Future<Item = Page, Error = FetchError> {
        let req = self.caller.query_params(pageid);
        self.caller
            .request(req)
            .and_then(|res| {
                if res.status() != 200 {
                    warn!("Got non 200 response from wikipedia: {:?}", res.status());
                }
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            .and_then(move |body| {
                let q: QueryResponse = serde_json::from_slice(&body)?;
                match q.query.pages.get(&pageid.to_string()) {
                    None => Err(FetchError::NoPage),
                    Some(p) => Ok(p.clone()),
                }
            })
    }

    pub fn get_page_views(
        &self,
        page_title: &str,
        month_retention: i64,
    ) -> impl Future<Item = u64, Error = FetchError> {
        let req = self.caller.get_pageviews_req(page_title, month_retention);
        self.caller
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<FetchError>()
            .and_then(|body| {
                let res: PageViewResponse = serde_json::from_slice(&body)?;
                let mut pageview: u64 = 0;
                for item in res.items {
                    pageview += item.views;
                }
                Ok(pageview)
            })
    }

    // separate lang with "|"
    // Ex: id|en
    pub fn get_wikidata(&self, wikidata_id: &str, lang: &str) -> impl Future<Item=WikiDataResponse, Error=FetchError> {
        let req = self.caller.wikidata_params(wikidata_id, lang);
        self.caller
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<FetchError>()
            .and_then(|body| {
                let res: WikiDataResponse = serde_json::from_slice(&body)?;
                Ok(res)
            })
    }

    // separate lang with "|"
    // Ex: id|en
    pub fn get_alias(&self, wikidata_id: &str, lang: &str) -> impl Future<Item=HashSet<String>, Error=FetchError>
    {
        let wikidata_id = wikidata_id.to_string();
        self.get_wikidata(&wikidata_id, lang)
            .and_then(move |resp| {
                let mut res: HashSet<String> = HashSet::new();
                let entity = match resp.entities.get(&wikidata_id) {
                    Some(e) => e,
                    None => return Err(FetchError::NoPage),
                };
                for (_, lang_alias) in entity.aliases.iter() {
                    for alias in lang_alias {
                        res.insert(alias.value.clone());
                    }
                }
                Ok(res)
            })
    }
}
