use gorgon::Lexer;
use gorgon::TokenKind;

fn main(){
    let code = "#Testando comentario
    fn fib(x Int) Int do 
        if x > 1 do
            fib(x-1) + fib(x-2)
        else
            x
        end
    end".to_string();
    let mut scanner = Lexer::new(&code);
    while let Ok(token) = scanner.next_token() {
        if token.kind == TokenKind::EndOfFile {break;}
        println!("{:?}",token);
    }
}   