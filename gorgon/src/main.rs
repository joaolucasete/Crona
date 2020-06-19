use gorgon::Lexer;
use gorgon::Parser;
use gorgon::TokenKind;

fn main() {
    let code = "a -= 2 a += 2 end".to_string();
    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    parser.advance().unwrap();
    parser.eat(TokenKind::End).unwrap();
}
