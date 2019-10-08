use wikipedia::Wikipedia;

fn main() {
    let w = Wikipedia::new("id");
    let page = w.get_page("titles", "Joko Widodo");
    println!("{:?}", page);
    // let pages = w.get_cat_members("Kategori:Politikus_Indonesia").unwrap();
    // for page in &pages {
    //     println!("{} {:?}", page.title, page.canonicalurl);
    // }
}
