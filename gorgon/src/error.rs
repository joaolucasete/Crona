use crate::Token;

#[derive(Debug)]
pub enum CompilerError {
    UnfinishedString,
    UnexpectedEOF,
    UnexpectedToken(Token),
}
