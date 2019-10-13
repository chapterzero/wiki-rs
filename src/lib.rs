mod request;
pub mod response;

use futures::future::Future;
use futures::stream::{self, Stream};
use request::Caller;
use response::{Namespace, Page, QueryResponse};

pub struct Wikipedia {
    caller: Caller,
}

impl Wikipedia {
    pub fn new(lang: &str) -> Wikipedia {
        Wikipedia {
            caller: Caller::new(lang),
        }
    }

    pub fn get_page(&self, pageid: u64) -> impl Future<Item = Page, Error = FetchError> {
        let req = self.caller.query_params(pageid);
        self.caller
            .client
            .request(req)
            .and_then(|res| {
                // asynchronously concatenate chunks of the body
                res.into_body().concat2()
            })
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(move |body| {
                // try to parse as json with serde_json
                let q: QueryResponse = serde_json::from_slice(&body)?;
                match q.query.pages.get(&pageid.to_string()) {
                    None => Err(FetchError::NoPage),
                    Some(p) => Ok(p.clone()),
                }
            })
    }

    pub fn get_cat_members(
        &self,
        cat_name: &str,
    ) -> impl Future<Item = Vec<Page>, Error = FetchError> + '_ {
        let pages = vec![];
        let cat_name = cat_name.to_string();
        stream::repeat(())
            .take(10)
            .fold(pages, move |mut pages, _| {
                self.get_cat_member(&cat_name, None)
                    .and_then(move |q| {
                        pages.push(Page {
                            pageid: 1,
                            title: "222".to_string(),
                            canonicalurl: "222".to_string(),
                            ns: Namespace::Page,
                            desc: None,
                            categories: None,
                        });
                        Ok(pages)
                    })
            })
    }

    fn get_cat_member(
        &self,
        cat_name: &str,
        cont_token: Option<String>,
    ) -> impl Future<Item = Page, Error = FetchError> {
        let req = self.caller.category_params(&cat_name, cont_token.as_ref());
        self.caller.client.request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<FetchError>()
            // use the body after concatenation
            .and_then(move |body| {
                // try to parse as json with serde_json
                let q: QueryResponse = serde_json::from_slice(&body)?;
                Ok(q.query.pages.get("1234").unwrap().clone())
            })
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
    NoPage,
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
