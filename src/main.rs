use toen::{lexer::Lexer, parser::Parser};

fn main() {
    let contents = std::fs::read_to_string("large.eton").unwrap();
    let parser = Lexer::new(&contents);

    println!("{}", parser.count());
}
