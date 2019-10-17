use crate::async_request::AsyncCaller;
use crate::errors::*;
use crate::response::Page;
use crate::response::{PageViewResponse, QueryResponse};
use futures::future::Future;
use futures::stream::Stream;
use log::{warn};

#[derive(Clone)]
pub struct WikipediaAsync {
    caller: AsyncCaller,
}

impl WikipediaAsync {
    pub fn new(lang: &str) -> WikipediaAsync {
        WikipediaAsync {
            caller: AsyncCaller::new(lang),
        }
    }

    pub fn get_page(&self, pageid: u64) -> impl Future<Item = Page, Error = FetchError> {
        let req = self.caller.query_params(pageid);
        self.caller
            .client
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
            .client
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
}
