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
    let page_fut = w.get_page(31706)
        .map(|p|{
            println!("{}: {}", p.pageid, p.title);
        })
        .map_err(|err| {
            println!("{:?}", err);
        });

    let page_name = page_name.clone();
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