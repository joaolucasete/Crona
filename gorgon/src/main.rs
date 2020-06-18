use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "3 & 2 == 123+2*2&3".to_string();

    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    println!("{:?}", parser.parse());
}
