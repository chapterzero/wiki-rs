pub mod response;
pub mod errors;
pub mod r#async;
mod async_request;
mod request;

use errors::*;
use response::{QueryResponse, Page};
use request::{Caller, PageId};
use reqwest::StatusCode;
use log::{debug};

pub struct ProxyConfig<'a>{
    pub host: &'a str,
    pub username: Option<&'a str>,
    pub password: Option<&'a str>,
}

pub struct Wikipedia {
    caller: Caller
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            caller: Caller::new(lang),
        }
    }

    pub fn get_page<T: PageId>(&self, pageid: T) -> Result<Page, FetchError> {
        let mut res = match self.caller.reqwest_client.execute(
            self.caller.query_params_sync(&pageid)
        ) {
            Ok(res) => res,
            Err(e) =>  {
                return Err(FetchError::Custom(format!("Error when executing request: {:?}", e)))
            }
        };

        if res.status() != StatusCode::OK {
            return Err(FetchError::Non200StatusCode)
        }

        let q: QueryResponse = match res.json() {
            Ok(s) => s,
            Err(e) => {
                return Err(FetchError::Custom(format!("Unable to parse response as json: {:?}", e)))
            }
        };

        match pageid.get_page_from_response(&q) {
            Some(p) => Ok(p),
            None => {
                return Err(FetchError::Custom(format!("Unable to get page using pageid {}", pageid)))
            }
        }
    }

    pub fn get_cat_members<T: PageId>(&self, pageid: T) -> Result<Vec<Page>, FetchError> {
        let mut pages = vec![];
        let mut gcmcontinue: Option<String> = None;

        loop {
            let mut res = match self.caller.reqwest_client.execute(
                self.caller.category_params(&pageid, gcmcontinue.as_ref())
            ) {
                Ok(res) => res,
                Err(e) =>  {
                    return Err(FetchError::Custom(format!("Error when executing request: {:?}", e)))
                }
            };

            let q: QueryResponse = match res.json() {
                Ok(s) => s,
                Err(e) => {
                    return Err(FetchError::Custom(format!("Unable to parse response as json: {:?}", e)))
                }
            };
            for (_, page) in &q.query.pages {
                pages.push(page.clone())
            }

            // continue if response contain continue token
            match q.cont {
                Some(cont) => {
                    match cont.gcmcontinue {
                        Some(cont_token) => {
                            debug!(
                                target: "Wikipedia", 
                                "Continuing because wikipedia returned continue token: {:?}",
                                cont_token
                            );
                            gcmcontinue = Some(cont_token);
                        }
                        None => break
                    }
                }
                None => break
            }
        }

        Ok(pages)
    }
}
