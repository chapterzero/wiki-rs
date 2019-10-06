use wikipedia::Wikipedia;
use wikipedia::response::Page;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::{self, Write};
use hyper::rt::{self, Future, Stream};

fn main() {
    // let w = Wikipedia::new("id");
    // let page = w.get_page("Jokox");
    // println!("{:?}", page);
    println!("{:?}", get_page())
}

fn get_page() {
    let url = "https://id.wikipedia.org/w/api.php?action=query&format=json&titles=Joko%20Widodo&prop=extracts|categories&exintro=true&explaintext=true&redirects=1&cllimit=20&clshow=!hidden".parse::<hyper::Uri>().unwrap();

    let url2 = "https://id.wikipedia.org/w/api.php?action=query&format=json&titles=Megawati&prop=extracts|categories&exintro=true&explaintext=true&redirects=1&cllimit=20&clshow=!hidden".parse::<hyper::Uri>().unwrap();

    let mut page = Page{
        pageid: 1,
        title: "XXX".to_string(),
        desc: "YYY".to_string(),
        categories: vec![],
    };

    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);
    let f1 = client
            .get(url)
            .and_then(move|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());

                page.pageid = 2;
                // The body is a stream, and for_each returns a new Future
                // when the stream is finished, and calls the closure on
                // each chunk of the body...
                res.into_body().for_each(|chunk| {
                    println!("PRINTING BODY");
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })

            })
            // If all good, just tell the user...
            .map(|_| {
                println!("\n\nDone.");
            })
            // If there was an error, let the user know...
            .map_err(|err| {
                eprintln!("Error {}", err);
            });

    let f2 = client
            .get(url2)
            .and_then(|res| {
                println!("Response: {}", res.status());
                println!("Headers: {:#?}", res.headers());

                // The body is a stream, and for_each returns a new Future
                // when the stream is finished, and calls the closure on
                // each chunk of the body...
                res.into_body().for_each(|chunk| {
                    println!("PRINTING BODY");
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })

            })
            // If all good, just tell the user...
            .map(|_| {
                println!("\n\nDone.");
            })
            // If there was an error, let the user know...
            .map_err(|err| {
                eprintln!("Error {}", err);
            });

    rt::run({
        let (a, b) = f1.join(f2).wait().unwrap();
        println!("{:?}", a)
    })
}
