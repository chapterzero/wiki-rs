use std::env;
use wikipedia::r#async::WikipediaAsync;
use futures::future::Future;
use tokio::runtime::{Builder};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: page name, Ex: Joko Widodo");

    let w = WikipediaAsync::new("id");
    let fut = w.get_page_views(page_name, 6)
        .map(|pv| {
            println!("{:?}", pv);
        })
        .map_err(|err| {
            println!("{:?}", err);
        });

    let mut rt = Builder::new()
        .core_threads(1)
        .build()
        .unwrap();

    rt.spawn(fut);
    rt.shutdown_on_idle()
        .wait().unwrap();
}
