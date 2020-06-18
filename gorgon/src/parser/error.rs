use crate::CompilerError;
use crate::Parser;

impl<'a> Parser<'a> {
    pub fn unexpected(&self) -> CompilerError {
        CompilerError::UnexpectedToken(self.actual.unwrap())
    }

    pub fn unexpected_eof(&self) -> CompilerError {
        CompilerError::UnexpectedEOF
    }
}
