use super::NodeKind;
use crate::BinKind;
use crate::CompilerError;
use crate::Node;
use crate::Parser;
use crate::Token;
use crate::TokenKind;
use std::boxed::Box;

// This module matches some common values for all the parts of the compiler
impl<'a> Parser<'a> {
    pub fn list(&mut self) -> Result<Vec<Node>, CompilerError> {
        let mut list = Vec::new();
        list.push(self.expr(1)?);
        while let Some(Token { kind: TokenKind::Comma, .. }) = self.next {
            self.advance()?;
            list.push(self.expr(1)?);
        }
        Ok(list)
    }

    // Names are identifiers connected for a dot
    pub fn name(&mut self) -> Result<Node, CompilerError> {
        use TokenKind::*;
        let mut spans = Vec::new();
        self.eat(Ident)?;
        spans.push(self.actual_span());
        while let Some(Token { kind: Dot, .. }) = self.next {
            self.advance()?;
            self.eat(Ident)?;
            spans.push(self.actual_span());
        }
        let mixed_span = Parser::mix_span(spans[0], spans[spans.len() - 1]);
        Ok(Node::new(NodeKind::Name(spans), mixed_span))
    }

    pub fn call(&mut self) -> Result<Node, CompilerError> {
        let name = self.name()?;

        if self.check_next(TokenKind::LPar) {
            self.advance()?;
            let args = if self.check_next(TokenKind::RPar) { Vec::new() } else { self.list()? };
            self.eat(TokenKind::RPar)?;
            let mixed_span = Parser::mix_span(name.span, self.actual_span());
            Ok(Node::new(NodeKind::Call(Box::new(name), args), mixed_span))
        } else {
            Ok(name)
        }
    }

    pub fn types(&mut self) -> Result<Node, CompilerError> {
        let name = self.name()?;
        if self.check_next(TokenKind::BinToken(BinKind::Less)) {
            self.advance()?;
            let generic = self.types()?;
            self.eat(TokenKind::BinToken(BinKind::Greater))?;
            let mixed_span = Parser::mix_span(name.span, self.actual_span());
            Ok(Node::new(NodeKind::Type(Box::new(name), Box::new(generic)), mixed_span))
        } else {
            Ok(name)
        }
    }
}
