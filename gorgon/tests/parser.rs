use gorgon::lexer::error::LexicalError;
use gorgon::Lexer;
use gorgon::Node;
use gorgon::Span;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_arith() -> Result<(), LexicalError> {
        let code = "a -= 2 a += 2 end".to_string();
        let scanner = Lexer::new(&code);
        let mut _parser = Parser::new(scanner);
    }
}
