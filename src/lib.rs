pub mod response;
mod request;

use response::{QueryResponse, Page};
use request::{Caller};
use futures::future::{Future};
use futures::stream::Stream;
use futures::future::{err, ok, Either};

pub struct Wikipedia {
    caller: Caller
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            caller: Caller::new(lang),
        }
    }

    // pub fn get_page<T: PageFrom>(&self, k: &'static str, from: T) -> Result<Page, Box<dyn Error>> {
    //     debug!(target: "Wikipedia", "Calling wikipedia API Query module...");
    //     let mut res = self.caller.client.execute(
    //         self.caller.query_params(k, from)
    //     )?;
    //     if res.status() != StatusCode::OK {
    //         return Err(GetError{
    //             from: (k, from.to_string()),
    //             reason: format!("expected status code 200, got {}", res.status())
    //         }.into())
    //     }

    //     let q: QueryResponse = res.json()?;
    //     let page = match from.extract_page(q) {
    //         Some(p) => p,
    //         None => {
    //             return Err(GetError{
    //                 from: (k, from.to_string()),
    //                 reason: "Unable to get page from result, either empty or wrong page id in pages".to_string(),
    //             }.into())
    //         }
    //     };

    //     return Ok(page)
    // }

    pub fn get_page(&self, pageid: u64) -> impl Future<Item=Page, Error=FetchError> {
        let req = self.caller.query_params(pageid);
        self.caller.client.request(req)
            .and_then(|res| {
                if !res.status().is_success() {
                    return Either::A(err(FetchError::Custom("Unable to get page from result".to_string())));
                }
                // asynchronously concatenate chunks of the body
                Either::B(res.into_body().concat2())
            })
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(move |body| {
                // try to parse as json with serde_json
                let q: QueryResponse = serde_json::from_slice(&body)?;
                match q.query.pages.get(&pageid.to_string()) {
                    None => Err(FetchError::Custom("Unable to get page from result".to_string())),
                    Some(p) => Ok(p.clone())
                }
            })
            .from_err()
    }

    // pub fn get_cat_members(&self, cat_name: &str) -> Result<Vec<Page>, Box<dyn Error>> {
    //     let mut pages = vec![];
    //     let mut gcmcontinue: Option<String> = None;

    //     loop {
    //         debug!(target: "Wikipedia", "Calling wikipedia API Query module...");
    //         let mut res = self.caller.client.execute(
    //             self.caller.category_params(cat_name, gcmcontinue.as_ref())
    //         )?;
    //         if res.status() != StatusCode::OK {
    //             return Err(GetError{
    //                 from: ("gcmtitle", cat_name.to_string()),
    //                 reason: format!("expected status code 200, got {}", res.status())
    //             }.into())
    //         }

    //         let q: QueryResponse = res.json()?;
    //         for (_, page) in &q.query.pages {
    //             pages.push(page.clone())
    //         }

    //         // continue if response contain continue token
    //         match q.cont {
    //             Some(cont) => {
    //                 match cont.gcmcontinue {
    //                     Some(cont_token) => {
    //                         debug!(
    //                             target: "Wikipedia", 
    //                             "Continuing because wikipedia returned continue token: {:?}",
    //                             cont_token
    //                         );
    //                         gcmcontinue = Some(cont_token);
    //                     }
    //                     None => break
    //                 }
    //             }
    //             None => break
    //         }
    //     }

    //     Ok(pages)
    // }

}

// #[derive(Debug)]
// pub struct GetError {
//     from: (&'static str, String),
//     reason: String,
// }

// impl fmt::Display for GetError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Error when Requesting with param {}: {}, Reason: {}",
//            self.from.0,
//            self.from.1,
//            self.reason,
//         )
//     }
// }

// impl Error for GetError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         None
//     }
// }

// Define a type so we can return multiple types of errors
#[derive(Debug)]
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
    Custom(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
