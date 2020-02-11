use std::env;
use wikipedia::r#async::WikipediaAsync;
use wikipedia::errors::FetchError;
use tokio::runtime::Runtime;
use futures::try_join;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: wikidata id");

    let mut rt = Runtime::new().unwrap();
    rt.block_on(process(page_name)).unwrap();
}

async fn process(page_name: &str) -> Result<(), FetchError> {
    let w = WikipediaAsync::new("id", None);
    let wikidata_fut = w.get_alias(page_name, "en|id");
    let page_fut = w.get_page(31706);
    let (p, wikibase) = try_join!(page_fut, wikidata_fut)?;
    println!("{}: {}", p.pageid, p.title);
    println!("Wikibase alias: {:?}", wikibase);
    Ok(())
}
