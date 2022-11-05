mod tokens;
mod text_processing;
mod latex;
mod tree;

use std::env::args;

use text_processing::parse_input;

fn main() {
    let argv: Vec<String> = Vec::from_iter(args());
    println!("{:?}", parse_input(argv[1].clone()));
}
