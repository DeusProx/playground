use std::fs;
mod parse;

fn main() {
    let website = fs::read_to_string("./assets/website.html").expect("html file should exist");
    let tokens = parse::tokenize(&website);
    println!("TOKENS:\n\n{:#?}", tokens);
}

