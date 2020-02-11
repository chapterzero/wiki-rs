#[macro_use]
extern crate log;

use wikipedia::Wikipedia;

fn main() {
    env_logger::init();
    let mut w = Wikipedia::new("id");
    let page = w.get_page("Joko Widodo");
    info!("Using &str");
    info!("{:?}", page);

    let page = w.get_page(31706u64);
    info!("Using u64");
    info!("{:?}", page);
}
