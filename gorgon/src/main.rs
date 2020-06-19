use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "a.b(b)".to_string();

    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    println!("{:?}", parser.parse());
}
