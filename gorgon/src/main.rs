use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "(1 + 2) * 3".to_string();

    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    parser.advance();
    println!("{:?}", parser.expr(1));
}
