use gorgon::scanner::LexicalError;
use gorgon::Scanner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_arith() -> Result<(), LexicalError> {
        use gorgon::TokenKind::*;
        use gorgon::BinKind::*;
        let input = &"123+456*78".to_string();
        let mut scanner = Scanner::new(input);
        assert_eq!(scanner.lex()?, Number(123));
        assert_eq!(scanner.lex()?, BinToken(Add));
        assert_eq!(scanner.lex()?, Number(456));
        assert_eq!(scanner.lex()?, BinToken(Mul));
        assert_eq!(scanner.lex()?, Number(78));
        Ok(())
    }
}
