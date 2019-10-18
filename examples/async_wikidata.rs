use std::env;
use wikipedia::r#async::WikipediaAsync;
use futures::future::Future;
use tokio::runtime::{Builder};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: wikidata id");

    let w = WikipediaAsync::new("id");
    let page_fut = w.get_alias(page_name, "en|id")
        .map(|p|{
            println!("{:?}", p);
        })
        .map_err(|err| {
            println!("{:?}", err);
        });

    let mut rt = Builder::new()
        .core_threads(2)
        .build()
        .unwrap();

    rt.spawn(page_fut);
    rt.shutdown_on_idle()
        .wait().unwrap();
}
