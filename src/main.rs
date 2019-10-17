use wikipedia::Wikipedia;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: page id, Ex: 1234");

    let w = Wikipedia::new("id");
    // let page = w.get_page(page_name.parse().expect("Unable to parse argument to u64"));
    // println!("{:?}", page);
    //
    let pages = w.get_cat_members(page_name).unwrap();
    for page in pages {
        println!("{}: {}", page.pageid, page.title);
    }
}
