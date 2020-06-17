pub mod error;
pub mod span;
pub use span::Span;
use std::str::Chars;
use error::LexicalError;

#[derive(Debug)]
pub struct Token{
    pub kind: TokenKind,
    pub span: Span
}

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
    Less,
    Greater,
    LessEqual,
    GreaterEqual
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Multiple Symbol Token
    BinToken(BinKind),
    Number,
    Ident,
    Str,
    // One Symbol Token
    LBraces,
    RBraces,
    LPar,
    RPar,
    LCurly,
    RCurly,
    Colon,
    Dot,

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
    End,
    Do,

    Comment,
    Whitespace,
    EndOfFile,
}

pub struct Lexer<'a> {
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

impl<'a> Lexer<'a> {
    pub fn new(text: &'a String) -> Lexer<'a> {
        Lexer {
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

    pub fn digest_identifier(&mut self, chr: char) -> TokenKind{
        use TokenKind::*;

        let mut id = String::new();
        id.push(chr);
        
        while let Some(chr) = self.first() {
            if !is_valid_ident(chr) {break}
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
            "end" => End,
            "do" => Do,
            _ => Ident
        }
    }

    pub fn digest(&mut self, predicative: impl Fn(char) -> bool ){
        while let Some(chr) = self.first() {
            if !predicative(chr) {
                break;
            }
            self.next();
        }
    }

    fn next_token_kind(&mut self) -> Result<TokenKind, LexicalError> {
        use TokenKind::*;
        use BinKind::*;
        match self.next() {
            Some(actual) => {
                let token = match actual {

                    '#' => {
                        self.digest(|x| x != '\n');
                        Comment
                    }

                    ' ' | '\t' | '\r' | '\n' => {
                        self.digest(|a| if let ' ' | '\t' | '\r' | '\n' = a { true } else { false });
                        Whitespace
                    },

                    // It matches just integers without dot
                    actual if is_digit(actual) => {self.digest(is_digit);Number},
                    // It matches keywords and identifier.
                    actual if is_valid_ident_start(actual) => {self.digest_identifier(actual)},
                    // TODO: Interpolation
                    '"' => {self.digest(|x| x != '"');Str},
                    '=' => double_token! (self, '=' => BinToken(Equal); Assign),
                    '+' => double_token! (self, '=' => AddEqual; BinToken(Add)),
                    '-' => double_token! (self, '=' => SubEqual; BinToken(Sub)),
                    '*' => double_token! (self, '=' => MulEqual; BinToken(Mul)),
                    '/' => double_token! (self, '=' => DivEqual; BinToken(Div)),
                    '<' => double_token! (self, '=' => BinToken(LessEqual); BinToken(Less)),
                    '>' => double_token! (self, '=' => BinToken(GreaterEqual); BinToken(Greater)),
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
                    '.' => Dot,
                    _ => {EndOfFile}
                };
                Ok(token)
            }
            None => Ok(EndOfFile)
        }
    }
    
    pub fn next_token(&mut self) -> Result<Token, LexicalError> {
        use TokenKind::*;
        loop {
            let start = self.consumed_length();
            let token = self.next_token_kind()?;
            if let Comment | Whitespace = token {continue;}
            return Ok( Token { kind: token, span: Span{ start, end: self.consumed_length()}})
        }
    }
}
