use wikipedia::Wikipedia;

fn main() {
    env_logger::init();
    let mut w = Wikipedia::new("id");
    let pages = w.get_cat_members("Kategori:Politikus_Indonesia").unwrap();
    println!("===========");
    let page2 = w.get_cat_members(69084u64).unwrap();

    println!("Len with string: {}, len with u64: {}", pages.len(), page2.len());
}


