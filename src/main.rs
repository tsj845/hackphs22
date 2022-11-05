mod tokens;
mod text_processing;
mod latex;

use std::env::args;

use text_processing::parse_input;

fn main() {
    let a: Vec<String> = Vec::from_iter(args());
    println!("{:?}", a);
    println!("{:?}", parse_input(a[1].clone()));
}
