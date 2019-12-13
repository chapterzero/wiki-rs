use std::env;
use wikipedia::r#async::WikipediaAsync;
use futures::future::Future;
use tokio::runtime::{Builder};
use wikipedia::ProxyConfig;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let page_name: String = args.get(1)
        .expect("Require 2nd argument: page name, Ex: Joko Widodo")
        .clone();

    let w = WikipediaAsync::new("id", None);
    let page_fut = w.get_page(page_name.clone())
        .map(|p|{
            println!("{}: {}", p.pageid, p.title);
        })
        .map_err(|err| {
            println!("{:?}", err);
        });

    let page_name = page_name;
    let fut = w.get_page_views(&page_name, 6)
        .map(move |pv| {
            println!("Page views for {}: {}", page_name, pv);
        })
        .map_err(|err| {
            println!("{:?}", err);
        });

    let mut rt = Builder::new()
        .core_threads(2)
        .build()
        .unwrap();

    rt.spawn(page_fut);
    rt.spawn(fut);
    rt.shutdown_on_idle()
        .wait().unwrap();
}
