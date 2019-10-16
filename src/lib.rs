pub mod response;
pub mod errors;
mod request;

use std::fmt;
use std::error::Error;
use errors::*;
use response::{QueryResponse, Page};
use request::Caller;
use futures::future::Future;
use reqwest::StatusCode;
use log::{debug};

pub struct Wikipedia {
    caller: Caller
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            caller: Caller::new(lang),
        }
    }

    pub fn get_page_sync(&self, pageid: u64) -> Result<Page, FetchError> {
        debug!(target: "Wikipedia", "Calling wikipedia API Query module...");
        let mut res = match self.caller.reqwest_client.execute(
            self.caller.query_params_sync(pageid)
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

        let page = match q.query.pages.get(&pageid.to_string()) {
            Some(p) => p.clone(),
            None => {
                return Err(FetchError::Custom(format!("Unable to get page using pageid {}", pageid)))
            }
        };

        return Ok(page)
    }

    pub fn get_cat_members(&self, cat_name: &str) -> Result<Vec<Page>, Box<dyn Error>> {
        let mut pages = vec![];
        let mut gcmcontinue: Option<String> = None;

        loop {
            debug!(target: "Wikipedia", "Calling wikipedia API Query module...");
            let mut res = self.caller.reqwest_client.execute(
                self.caller.category_params(cat_name, gcmcontinue.as_ref())
            )?;
            if res.status() != StatusCode::OK {
                return Err(GetError{
                    from: ("gcmtitle", cat_name.to_string()),
                    reason: format!("expected status code 200, got {}", res.status())
                }.into())
            }

            let q: QueryResponse = res.json()?;
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

    pub fn get_page_views(page_title: &str) -> impl Future<Item=(), Error=FetchError> {
        futures::future::ok(())
    }

}

#[derive(Debug)]
pub struct GetError {
    from: (&'static str, String),
    reason: String,
}

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error when Requesting with param {}: {}, Reason: {}",
           self.from.0,
           self.from.1,
           self.reason,
        )
    }
}

impl Error for GetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
