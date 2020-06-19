use crate::Token;

#[derive(Debug)]
pub enum CompilerError {
    UnfinishedString,
    UnexpectedEOF,
    NotRecognizableChar,
    UnexpectedToken(Token),
}
