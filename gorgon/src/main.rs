use gorgon::Lexer;
use gorgon::Parser;
use std::fs;

fn main() {
    let filename = "gorgon/code_tests/fib.gn";
    let text = fs::read_to_string(filename).expect("Cannot read file");
    let scanner = Lexer::new(&text);
    let mut parser = Parser::new(scanner);
    println!("{:?}", parser.parse());
}
