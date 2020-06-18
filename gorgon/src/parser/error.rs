use crate::CompilerError;
use crate::Parser;
use crate::TokenKind;

impl<'a> Parser<'a> {
    pub fn unexpected(&self, kind: TokenKind) -> CompilerError {
        CompilerError::UnexpectedToken(kind, self.actual.unwrap())
    }

    pub fn unexpected_eof(&self) -> CompilerError {
        CompilerError::UnexpectedEOF
    }
}
