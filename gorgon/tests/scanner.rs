use gorgon::lexer::error::LexicalError;
use gorgon::Lexer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_arith() -> Result<(), LexicalError> {
        use gorgon::TokenKind::*;
        use gorgon::BinKind::*;
        let input = &"123+456*78".to_string();
        let mut scanner = Lexer::new(input);
        assert_eq!(scanner.next_token()?.kind, Number);
        assert_eq!(scanner.next_token()?.kind, BinToken(Add));
        assert_eq!(scanner.next_token()?.kind, Number);
        assert_eq!(scanner.next_token()?.kind, BinToken(Mul));
        assert_eq!(scanner.next_token()?.kind, Number);
        Ok(())
    }

    #[test]
    fn scan_fib() -> Result<(), LexicalError> {
        use gorgon::TokenKind::*;
        use gorgon::BinKind::*;

        let code = "#Testando comentario
        fn fib(x Int) Int do 
            if x > 1 do
                fib(x-1) + fib(x-2)
            else
                x
            end
        end".to_string();

        let mut scanner = Lexer::new(&code);
        assert_eq!(scanner.next_token()?.kind, Fn);
        assert_eq!(scanner.next_token()?.kind, Ident);
        assert_eq!(scanner.next_token()?.kind, LPar);
        assert_eq!(scanner.next_token()?.kind, Ident);
        assert_eq!(scanner.next_token()?.kind, Ident);
        assert_eq!(scanner.next_token()?.kind, RPar);
        assert_eq!(scanner.next_token()?.kind, Ident);
        assert_eq!(scanner.next_token()?.kind, Do);
        assert_eq!(scanner.next_token()?.kind, If);
        assert_eq!(scanner.next_token()?.kind, Ident);
        assert_eq!(scanner.next_token()?.kind, BinToken(Greater));
        assert_eq!(scanner.next_token()?.kind, Number);
        assert_eq!(scanner.next_token()?.kind, Do);
        Ok(())
    }
}
