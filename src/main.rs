use wikipedia::Wikipedia;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let page_name: &String = args.get(1)
        .expect("Require 2nd argument: page_name, Ex: Joko Widodo");

    let w = Wikipedia::new("id");
    let page = w.get_page("titles", page_name.as_ref());
    println!("{:?}", page);
}
