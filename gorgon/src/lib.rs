#![feature(or_patterns)]

pub mod lexer;
pub use lexer::TokenKind;
pub use lexer::BinKind;
pub use lexer::Lexer;