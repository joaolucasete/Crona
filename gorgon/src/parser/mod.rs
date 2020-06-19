use crate::BinKind;
use crate::CompilerError;
use crate::Lexer;
use crate::Span;
use crate::Token;
use crate::TokenKind;

use std::boxed::Box;

mod error;
mod expr;
mod vals;

#[derive(Debug)]
pub enum NodeKind {
    Binary(Box<Node>, BinKind, Box<Node>),
    Name(Vec<Span>),
    Call(Box<Node>, Vec<Node>),
    Number,
    None,
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    span: Span,
}

impl Node {
    pub fn new(kind: NodeKind, span: Span) -> Node {
        Node { kind, span }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    next: Option<Token>,
    actual: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            next: None,
            actual: None,
        }
    }

    pub fn advance(&mut self) -> Result<Option<Token>, CompilerError> {
        self.actual = self.next;
        self.next = Some(self.lexer.next_token()?);
        Ok(self.actual)
    }

    pub fn eat(&mut self, check_kind: TokenKind) -> Result<Option<Token>, CompilerError> {
        let next = self.advance()?;
        if let Some(Token { kind, .. }) = next {
            if kind == check_kind {
                Ok(next)
            } else {
                Err(self.unexpected())
            }
        } else {
            Err(self.unexpected_eof())
        }
    }

    pub fn check_next(&mut self,check_kind: TokenKind) -> bool {
        if let Some(Token { kind, .. }) = self.next {
            if kind == check_kind {
                true
            } else{
                false
            }
        }else{
            false
        }
    }

    pub fn mix_span(start_span: Span, end_span: Span) -> Span {
        Span {
            start: start_span.start,
            end: end_span.end,
        }
    }

    pub fn actual_span(&mut self) -> Span {
        match self.actual {
            Some(tkn) => tkn.span,
            None => Span { start: 0, end: 0 },
        }
    }
    pub fn parse(&mut self) -> Result<Node, CompilerError> {
        self.advance()?;
        self.call()
    }
}
