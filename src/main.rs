use wikipedia::{Wikipedia, FetchError};
use std::env;
use hyper::rt::{self, Future};
use tokio::runtime::{Runtime, Builder};
use futures::stream;
use futures::stream::Stream;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let page_name: String = args.get(1)
        .expect("Require 2nd argument: page_name, Ex: Joko Widodo")
        .clone();

    let w = Wikipedia::new("id");
    // let pageids: Vec<u64> = vec![31706, 3483, 2834, 6181, 31706, 3483, 2834, 6181];

    // let fut = stream::iter_ok(pageids).for_each(move |pageid| {
    //     let pfut = w.get_page(pageid)
    //         .map(|page| {
    //             println!("{}: {}", page.pageid, page.title);
    //         })
    //         .map_err(|e| {
    //             println!("Got err {:?}", e)
    //         });
    //     tokio::spawn(pfut)
    // });
    //
    let fut = w.get_cat_members("Kategori:Politikus_Indonesia")
             .map(|pages| {
                 println!("{:?}", pages);
             })
             .map_err(|e| {
                 println!("Got err {:?}", e)
             });

    let mut rt = Builder::new()
        .core_threads(4)
        .build()
        .unwrap();

    rt.spawn(fut);
    rt.shutdown_on_idle()
        .wait().unwrap();
    println!("{} ms", now.elapsed().as_millis())


    // let fut2 = w.get_page(3483)
    //     .map(|page| {
    //         println!("{:?}", page);
    //     })
    //     .map_err(|e| {
    //         match e {
    //             FetchError::Http(e) => println!("http error: {}", e),
    //             FetchError::Json(e) => println!("json parsing error: {}", e),
    //             FetchError::Custom(e) => println!("Wikipedia error: {}", e),
    //         }
    //     });

    // let mut rt = Runtime::new().unwrap();
    // rt.spawn(fut);
    // rt.spawn(fut2);

    // rt.shutdown_on_idle()
    //     .wait().unwrap();
}
