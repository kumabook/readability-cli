extern crate readability;
extern crate url;

use std::env;
use readability::extractor;
use url::Url;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        print_usage();
        return
    }
    match Url::parse(&args[1]) {
        Ok(url) => match extractor::scrape(&url.as_str()) {
            Ok(product) => {
                println!("{}", product.content);
            },
            Err(_) => println!("error occured"),
        }
        Err(_) => print_usage(),
    }
}

fn print_usage() {
    println!("Usage: readability <url>")
}
