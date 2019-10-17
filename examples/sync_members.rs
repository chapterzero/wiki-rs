use wikipedia::Wikipedia;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: cat name, Ex: Katgori:Politikus_Indonesia");

    let w = Wikipedia::new("id");
    let pages = w.get_cat_members(page_name).unwrap();
    for page in pages {
        println!("{}: {}", page.pageid, page.title);
    }
}


