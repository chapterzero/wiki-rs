pub mod response;
mod request;

use std::fmt;
use std::error::Error;
use response::{QueryResponse, Page};
use request::{Caller, PageFrom};
use reqwest::{Client as HttpClient, StatusCode};

pub struct Wikipedia {
    caller: Caller
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            caller: Caller {
                base_api_url: format!("https://{}.wikipedia.org/w/api.php", lang),
                client: HttpClient::new(),
            }
        }
    }

    pub fn get_page<T: PageFrom>(&self, k: &'static str, from: T) -> Result<Page, Box<dyn Error>> {
        let mut res = self.caller.client.execute(
            self.caller.query_params(k, from)
        )?;
        if res.status() != StatusCode::OK {
            return Err(GetError{
                from: (k, from.to_string()),
                reason: format!("expected status code 200, got {}", res.status())
            }.into())
        }

        let q: QueryResponse = res.json()?;
        let page = match from.extract_page(q) {
            Some(p) => p,
            None => {
                return Err(GetError{
                    from: (k, from.to_string()),
                    reason: "Unable to get page from result, either empty or wrong page id in pages".to_string(),
                }.into())
            }
        };

        return Ok(page)
    }

    pub fn get_category_members<T: PageFrom>(&self, from: T) -> Result<Vec<Page>, Box<dyn Error>> {
        Ok(vec![])
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
