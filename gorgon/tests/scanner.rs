use gorgon::scanner::LexicalError;
use gorgon::Scanner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_arith() -> Result<(), LexicalError> {
        use gorgon::Token::*;
        let input = &"123+456*78".to_string();
        let mut scanner = Scanner::new(input);
        assert_eq!(scanner.lex()?, Number);
        assert_eq!(scanner.lex()?, Add);
        assert_eq!(scanner.lex()?, Number);
        assert_eq!(scanner.lex()?, Mul);
        assert_eq!(scanner.lex()?, Number);
        Ok(())
    }
}
