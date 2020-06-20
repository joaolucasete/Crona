use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "
    if x > 2 do
        abc := 3
    end"
    .to_string();
    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    println!("{:#?}", parser.parse());
}
