use std::env;
use wikipedia::r#async::WikipediaAsync;
use wikipedia::errors::FetchError;
use tokio::runtime::Runtime;
use futures::try_join;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let page_name: String = args.get(1)
        .expect("Require 2nd argument: page name, Ex: Joko Widodo")
        .clone();
    let mut rt = Runtime::new().unwrap();
    rt.block_on(process(&page_name)).unwrap();
}

async fn process(page_name: &str) -> Result<(), FetchError> {
    let lang = wikipedia::Lang::EN;
    let w = WikipediaAsync::new(None);
    let page_fut = w.get_page(page_name, &lang);
    let page_views_fut = w.get_page_views(&page_name, 6, &lang);
    let (p, pv) = try_join!(page_fut, page_views_fut)?;
    println!("{}: {}", p.pageid, p.title);
    println!("Page views for {}: {}", page_name, pv);
    Ok(())
}
