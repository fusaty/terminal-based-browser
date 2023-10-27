use reqwest;
use select::document::Document;
use select::predicate::Name;

fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

fn parse_and_display_html(html: &str) {
    let document = Document::from(html);

    for node in document.find(Name("a")) {
        if let text = node.text() {
            println!("{}", text);
        }
    }
}

fn main() {
    loop {
    let mut url = String::new();
    println!("Enter the website URL: ");
    std::io::stdin().read_line(&mut url).unwrap();

    match fetch_html(&url) {
        Ok(html) => {
            parse_and_display_html(&html);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
}