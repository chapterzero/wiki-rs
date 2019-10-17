use crate::async_request::AsyncCaller;
use crate::errors::*;
use futures::future::Future;

pub struct WikipediaAsync {
    caller: AsyncCaller,
}

impl WikipediaAsync {
    pub fn new(lang: &str) -> WikipediaAsync {
        WikipediaAsync {
            caller: AsyncCaller::new(lang),
        }
    }

    pub fn get_page_views(
        &self,
        page_title: &str,
        month_retention: i64,
    ) -> impl Future<Item = (), Error = FetchError> {
        let page_url = self.caller.get_pageviews_url(page_title, month_retention);
        println!("{}", page_url);
        futures::future::ok(())
    }
}
