use super::Node;
use super::NodeKind;
use crate::CompilerError;
use crate::Parser;
use crate::TokenKind;
use std::boxed::Box;

impl<'a> Parser<'a> {
    pub fn function(&mut self) -> Result<Node, CompilerError> {
        self.eat(TokenKind::Fn)?;
        let name = self.eat(TokenKind::Ident)?.span;
        self.eat(TokenKind::LPar)?;
        let mut args = Vec::new();
        while self.check_next(TokenKind::Ident) {
            let name = self.eat(TokenKind::Ident)?.span;
            let kind = Box::new(self.types()?);
            args.push((name, kind));
        }
        self.eat(TokenKind::RPar)?;

        let kind = Box::new(self.types()?);

        self.eat(TokenKind::Do)?;
        let compound = self.compound(&|x| x == TokenKind::End)?;
        self.advance()?;

        Ok(Node::new(
            NodeKind::Function {
                name,
                args,
                kind,
                compound: Box::new(compound),
            },
            Parser::mix_span(name, self.actual_span()),
        ))
    }
}
