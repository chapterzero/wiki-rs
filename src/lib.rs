pub mod response;
pub mod errors;
pub mod r#async;
mod async_request;
mod request;

use errors::*;
use response::{Page};
use r#async::WikipediaAsync;
use tokio::runtime::Runtime;
use request::{PageId};

pub struct ProxyConfig<'a>{
    pub host: &'a str,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

pub struct Wikipedia {
    async_lib: WikipediaAsync,
    rt: Runtime,
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            async_lib: WikipediaAsync::new(lang, None),
            rt: Runtime::new().unwrap(),
        }
    }

    pub fn get_page<T: PageId>(&mut self, pageid: T) -> Result<Page, FetchError> {
        self.rt.block_on(self.async_lib.get_page(pageid))
    }

    pub fn get_cat_members<T: PageId>(&mut self, pageid: T) -> Result<Vec<Page>, FetchError> {
        self.rt.block_on(self.async_lib.get_cat_members(pageid))
    }
}
