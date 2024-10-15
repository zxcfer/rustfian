use std::fs;
use regex::Regex;

fn main() {
    // Read the contents of the HTML file
    let html_content = fs::read_to_string("demo.html").expect("Failed to read the file");

    // Remove span tags and keep the content
    let cleaned_content = remove_span_tags(&html_content);

    // Write the cleaned content back to the file
    fs::write("demo1.html", cleaned_content).expect("Failed to write to the file");
}

fn remove_span_tags(html: &str) -> String {
    lazy_static::lazy_static! {
        static ref SPAN_REGEX: Regex = Regex::new(r"<span(.*?)>(.*?)</span>").unwrap();
    }

    SPAN_REGEX.replace_all(html, "$2").to_string()
}
