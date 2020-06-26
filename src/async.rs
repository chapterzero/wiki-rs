use crate::async_request::AsyncCaller;
use crate::errors::*;
use crate::response::Page;
use crate::response::*;
use crate::request::PageId;
use crate::ProxyConfig;
use bytes::{BytesMut, BufMut};
use hyper::{body::HttpBody};
use std::collections::HashSet;
use log::{warn, debug};
use super::Lang;

#[derive(Clone)]
pub struct WikipediaAsync {
    caller: AsyncCaller,
}

impl WikipediaAsync {
    pub fn new(proxy: Option<ProxyConfig>) -> WikipediaAsync {
        WikipediaAsync {
            caller: AsyncCaller::new(proxy),
        }
    }

    pub async fn get_page<T: PageId>(&self, pageid: T, lang: &Lang) -> Result<Page, FetchError> {
        let req = self.caller.query_params(&pageid, lang);
        let mut res = self.caller.request(req).await?;
        if res.status() != 200 {
            warn!("Got non 200 response from wikipedia: {:?}", res.status());
        }
        let mut body = BytesMut::with_capacity(1024);
        while let Some(next) = res.data().await {
            body.put(next?);
        }
        let q: QueryResponse = serde_json::from_slice(&body)?;
        match pageid.get_page_from_response(&q) {
            None => Err(FetchError::NoPage),
            Some(p) => Ok(p),
        }
    }

    pub async fn get_page_views(
        &self,
        page_title: &str,
        month_retention: i64,
        lang: &Lang,
    ) -> Result<u64, FetchError> {
        let mut res = self.caller.request(self.caller.get_pageviews_req(page_title, month_retention, lang)).await?;
        let mut body = BytesMut::with_capacity(1024);
        while let Some(next) = res.data().await {
            body.put(next?);
        }
        let res: PageViewResponse = serde_json::from_slice(&body)?;
        let mut pageview: u64 = 0;
        for item in res.items {
            pageview += item.views;
        }
        Ok(pageview)
    }

    // separate lang with "|"
    // Ex: id|en
    pub async fn get_wikidata(&self, wikidata_id: &str, lang: &str) -> Result<WikiDataResponse, FetchError> {
        let mut res = self.caller.request(self.caller.wikidata_params(wikidata_id, lang)).await?;
        let mut body = BytesMut::with_capacity(1024);
        while let Some(next) = res.data().await {
            body.put(next?);
        }
        let res: WikiDataResponse = serde_json::from_slice(&body)?;
        Ok(res)
    }

    // separate lang with "|"
    // Ex: id|en
    pub async fn get_alias(&self, wikidata_id: &str, lang: &str) -> Result<HashSet<String>, FetchError>
    {
        let wikidata_id = wikidata_id.to_string();
        let resp = self.get_wikidata(&wikidata_id, lang).await?;
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
    }

    pub async fn get_cat_members<T: PageId>(&self, pageid: T, lang: &Lang) -> Result<Vec<Page>, FetchError> {
        let mut pages = vec![];
        let mut gcmcontinue: Option<String> = None;
        loop {
            let mut resp = self.caller.request(self.caller.category_params(&pageid, lang, gcmcontinue.as_ref())).await?;
            let mut body = BytesMut::with_capacity(1024);
            while let Some(next) = resp.data().await {
                body.put(next?);
            }
            let q: QueryResponse = serde_json::from_slice(&body)?;
            for (_, page) in q.query.pages {
                pages.push(page)
            }
            if let Some(cont) = q.cont {
                if let Some(token) = cont.gcmcontinue {
                    debug!(
                        target: "Wikipedia", 
                        "Continuing because wikipedia returned continue token: {:?}",
                        token
                    );
                    gcmcontinue = Some(token);
                } else {
                    break
                }
            } else {
                break
            }
        }
        Ok(pages)
    }
}
