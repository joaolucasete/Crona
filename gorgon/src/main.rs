use gorgon::Lexer;
use gorgon::Parser;

fn main() {
    let code = "
    fn fib(x Int<Ke>) Int do 
        variable := 2 + 3
    end"
    .to_string();
    let scanner = Lexer::new(&code);
    let mut parser = Parser::new(scanner);
    println!("{:#?}", parser.parse());
}
