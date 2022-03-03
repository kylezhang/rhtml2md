extern crate rhtml2md;

use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).expect("Must be readable HTML!");
    println!("{}", rhtml2md::parse_html(&buffer));
}