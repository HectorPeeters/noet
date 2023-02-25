use toen::parser::Parser;

fn main() {
    let parser = Parser::new(include_str!("../test.eton"));

    for element in parser {
        println!("{element:#?}");
    }
}
