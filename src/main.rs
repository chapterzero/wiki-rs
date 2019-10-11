use wikipedia::{Wikipedia, FetchError};
use std::env;
use hyper::rt::{self, Future};

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_name: String = args.get(1)
        .expect("Require 2nd argument: page_name, Ex: Joko Widodo")
        .clone();

    let w = Wikipedia::new("id");
    let fut = w.get_page(31706)
        .map(|page| {
            println!("{:?}", page);
        })
        .map_err(|e| {
            match e {
                FetchError::Http(e) => println!("http error: {}", e),
                FetchError::Json(e) => println!("json parsing error: {}", e),
                FetchError::Custom(e) => println!("Wikipedia error: {}", e),
            }
        });

    rt::run(rt::lazy(||{
        fut
    }));
}
