use wikipedia::Wikipedia;

fn main() {
    let w = Wikipedia::new("id");
    let page = w.get_page("Jokox");
    println!("{:?}", page);
}
