mod tokens;
mod text_processing;
mod latex;
mod tree;

use std::env::args;

use text_processing::parse_input;

use crate::latex::to_latex;

fn main() {
    let argv: Vec<String> = Vec::from_iter(args());
    let toks = parse_input((&argv[1..].concat()).to_owned());
    let ex = Box::new(crate::tree::Operation::new(toks));
    println!("{}", to_latex(&*ex));
    println!("{}", ex.calculate());
}
