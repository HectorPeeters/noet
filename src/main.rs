use toen::{lexer::Lexer, parser::Parser};

fn main() {
    let mut parser = Parser::new(include_str!("../test.eton"));

    while let Some(element) = parser.next() {
        println!("{element:#?}");
    }
}
