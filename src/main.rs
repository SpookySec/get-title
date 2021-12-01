use std::env::args;

use ureq;
use scraper::{Html, Selector};

fn get_title(html: &String) -> String {
    let doc = Html::parse_document(&html);
    let title = doc.select(&Selector::parse("title").unwrap()).next().unwrap();
    title.text().collect::<Vec<_>>().join("")
}

fn main() {

    // check if the user provided a URL
    let url = match args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: {} <url>", args().nth(0).unwrap());
            return;
        }
    };

    // get the response
    let res = ureq::get(&url).call().unwrap();

    match res.status() {
        200 => {
            // get the HTML
            let html = res.into_string().unwrap();

            // get the title
            let title = get_title(&html);

            // print the title
            println!("{}", title);
        },
        _ => {
            println!("Error: {}", res.status());
        }
    }
}
