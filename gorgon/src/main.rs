use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "2 > 3 and 2 < 4".to_string();
    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    parser.advance().unwrap();
    println!("{:?}", parser.expr(1));
}
