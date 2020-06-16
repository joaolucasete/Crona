use std::io::Error;
use std::str::Chars;

mod error;
pub use error::LexicalError;

macro_rules! double_token {
    ($self:ident, $pattern:pat => $first:expr; $other:expr) => {
        if let Some($pattern) = $self.first() {
            $first
        } else {
            $other
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum BinKind {
    Add,
    Mul,
    Sub,
    Div,
    And,
    Or,
    Xor,
    Equal,
    AndCmp,
    OrCmp,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Multiple Symbol Token
    BinToken(BinKind),
    Number(u32),
    Ident(u32),
    Str,
    // One Symbol Token
    LBraces,
    RBraces,
    LPar,
    RPar,
    LCurly,
    RCurly,
    Colon,

    // Two Symbol Token
    AddEqual,
    MulEqual,
    SubEqual,
    DivEqual,
    Assign,
    Short,

    // Keywords
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

    // TODO: Read from file

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

    pub fn digest_number(&mut self, chr: char) -> TokenKind{
        use TokenKind::*;
        let mut num = chr as u32 - 48;
        while let Some(chr) = self.first() {
            if !is_digit(chr) {break}
            num *= 10;
            num += chr as u32 - 48;
            self.next();
        }
        Number(num)
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
            "and" => BinToken(BinKind::AndCmp),
            "or" => BinToken(BinKind::OrCmp),
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
            _ => Ident(0)
        }
    }

    pub fn lex(&mut self) -> Result<TokenKind, LexicalError> {
        use TokenKind::*;
        use BinKind::*;
        match self.next() {
            Some(actual) => {
                let token = match actual {
                    // It matches just integers without dot
                    actual if is_digit(actual) => {self.digest_number(actual)},
                    // It matches keywords and identifier.
                    actual if is_valid_ident_start(actual) => {self.digest_identifier(actual)},
                    // TODO: Interpolation
                    '"' => {
                        //self.digest(|s| s != '"');
                        self.next();
                        Str
                    },

                    '=' => double_token! (self, '=' => BinToken(Equal); Assign),
                    '+' => double_token! (self, '=' => AddEqual; BinToken(Add)),
                    '-' => double_token! (self, '=' => SubEqual; BinToken(Sub)),
                    '*' => double_token! (self, '=' => MulEqual; BinToken(Mul)),
                    '/' => double_token! (self, '=' => DivEqual; BinToken(Div)),
                    ':' => double_token! (self, '=' => Short; Colon),
                    '&' => BinToken(And),
                    '|' => BinToken(Or),
                    '^' => BinToken(Xor),
                    '(' => LPar,
                    ')' => RPar,
                    '[' => LBraces,
                    ']' => RBraces,
                    '{' => LCurly,
                    '}' => RCurly,
                    _ => {EndOfFile}
                };
                Ok(token)
            }
            None => Ok(EndOfFile)
        }
    }
}
