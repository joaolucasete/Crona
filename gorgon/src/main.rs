use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "a<x>".to_string();
    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    parser.advance().unwrap();
    println!("{:?}", parser.types());
}
