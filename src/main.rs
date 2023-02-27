use std::time::Instant;

use noet::{lexer::Lexer, parser::Parser};

fn main() {
    let start = Instant::now();
    let contents = std::fs::read_to_string("large.eton").unwrap();
    println!("{:?}", Instant::now().duration_since(start));
    let parser = Parser::new(&contents);

    println!("{:#?}", parser.count());
}
