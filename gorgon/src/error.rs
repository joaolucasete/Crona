use crate::Token;
use crate::TokenKind;

#[derive(Debug)]
pub enum CompilerError {
    UnfinishedString,
    UnexpectedEOF,
    UnexpectedToken(TokenKind, Token),
}
