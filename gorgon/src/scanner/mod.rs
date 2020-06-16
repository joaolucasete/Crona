use std::io::Error;
use std::str::Chars;

mod error;
pub use error::LexicalError;

macro_rules! double_token {
    ($self:ident, $pattern:pat => $first:expr; $other:expr) => {
        match $self.first() {
            Some($pattern) => $first,
            _ => $other
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Multiple Symbol Token
    Number,
    Ident,
    Str,
    
    // One Symbol Token
    Add,
    Mul,
    Sub,
    Div,
    And,
    Or,
    Xor,
    Equal,

    // Two Symbol Token
    AddEqual,
    MulEqual,
    SubEqual,
    DivEqual,
    Assign,

    // Keywords
    AndCmp,
    OrCmp,
    Mut,
    Let,
    If,
    Elif,
    Else,
    For,
    Loop,
    Send,
    Receive,
    Fn,

    EndOfFile,
}

pub struct Scanner<'a> {
    initial: usize,
    text: Chars<'a>,
    prev: char
}

fn is_digit(character: char) -> bool {
    ('0' ..= '9').contains(&character)
}

fn is_valid_ident_start(character: char) -> bool {
    match character{
        'a' ..= 'z' | 'A' ..= 'Z' | '_' => true,
        _ => false
    }
}

fn is_valid_ident(character: char) -> bool {
    match character{
        'a' ..= 'z' | 'A' ..= 'Z' | '_' | '0' ..= '9' => true,
        _ => false
    }
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a String) -> Scanner<'a> {
        Scanner {
            initial: text.len(),
            text: text.chars(),
            prev: '\0'
        }
    }

    pub fn from_file(file_name: &'a String) -> Result<Scanner<'a>, Error> {
        Ok(Scanner {
            initial: 0,
            text: file_name.chars(),
            prev: '\0'
        })
    }

    pub fn next(&mut self) -> Option<char> {
        let actual = self.text.nth(0)?;
        self.prev = actual;
        Some(actual)
    }

    pub fn first(&mut self) -> Option<char> {
        self.text.clone().nth(0)
    }

    pub fn consumed_length(&mut self) -> usize{
        self.initial - self.text.as_str().len()
    }

    pub fn digest(&mut self, predicative: impl Fn(char) -> bool ){
        while let Some(chr) = self.first() {
            if !predicative(chr) {
                break;
            }
            self.next();
        }
    }

    pub fn digest_identifier(&mut self, chr: char) -> TokenKind{
        use TokenKind::*;

        let mut id = String::new();
        id.push(chr);
        
        while let Some(chr) = self.first() {
            if !is_valid_ident_start(chr) {break}
            id.push(chr);
            self.next();
        }

        match id.as_str() {
            "and" => AndCmp,
            "or" => OrCmp,
            "mut" => Mut,
            "let" => Let,
            "if" => If,
            "elif" => Elif,
            "else" => Else,
            "for" => For,
            "loop" => Loop,
            "send" => Send,
            "receive" => Receive,
            "fn" => Fn,
            _ => Ident
        }
    }

    pub fn lex(&mut self) -> Result<TokenKind, LexicalError> {
        use TokenKind::*;
        match self.next() {
            Some(actual) => {
                let token = match actual {
                    // It matches just integers without dot
                    actual if is_digit(actual) => {
                        self.digest(is_digit);
                        Number
                    },

                    // It matches keywords and identifier.
                    actual if is_valid_ident_start(actual) => {
                        self.digest_identifier(actual)
                    },

                    // TODO: Interpolation
                    '"' => {
                        self.digest(|s| s != '"');
                        self.next();
                        Str
                    },

                    '=' => double_token! (self, '=' => Assign; Equal),
                    '+' => double_token! (self, '=' => AddEqual; Add),
                    '-' => double_token! (self, '=' => SubEqual; Sub),
                    '*' => double_token! (self, '=' => MulEqual; Mul),
                    '/' => double_token! (self, '=' => DivEqual; Div),
                    '&' => And,
                    '|' => Or,
                    '^' => Xor,
                    _ => {EndOfFile}
                };
                Ok(token)
            }
            None => Ok(EndOfFile)
        }
    }
}
