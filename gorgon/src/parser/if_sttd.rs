use super::Node;
use super::NodeKind;
use crate::CompilerError;
use crate::Parser;
use crate::TokenKind;
use std::boxed::Box;

impl<'a> Parser<'a> {
    pub fn if_statement(&mut self) -> Result<Node, CompilerError> {
        self.eat(TokenKind::If)?;
        let start = self.actual_span();
        let cond = self.expr(1)?;
        self.eat(TokenKind::Do)?;
        let compound = self.compound(&|x| x == TokenKind::End || x == TokenKind::Else || x == TokenKind::Elif)?;
        let mut elifs = Vec::new();

        while self.check_next(TokenKind::Elif) {
            let cond = self.expr(1)?;
            self.eat(TokenKind::Do)?;
            let compound = self.compound(&|x| x == TokenKind::End || x == TokenKind::Else || x == TokenKind::Elif)?;
            elifs.push((Box::new(cond),Box::new(compound)))
        }

        let else_compound = if self.check_next(TokenKind::Else) {
            self.advance()?;
            Some(Box::new(self.compound(&|x| x == TokenKind::End)?))
        } else{
            None
        };
        
        self.eat(TokenKind::End)?;
        
        Ok(Node::new(
            NodeKind::Condition {
                ifsttd: (Box::new(cond),Box::new(compound)),
                elif: Vec::new(),
                else_compound
            },
            Parser::mix_span(start, self.actual_span()),
        ))
    }
}
