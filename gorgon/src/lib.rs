#![feature(or_patterns)]

pub mod scanner;
pub use scanner::Scanner;
pub use scanner::TokenKind;
pub use scanner::BinKind;