use super::NodeKind;
use crate::BinKind;
use crate::CompilerError;
use crate::Node;
use crate::Parser;
use crate::Token;
use crate::TokenKind;
use std::boxed::Box;

// All the non associative operations have to be isolated to not cause 
// Strange behaviour. So this function map all the non associative operations

fn not_associative(token: BinKind) -> bool{
    use BinKind::*;
    match token {
        Diff | Equal | Less | Greater | GreaterEqual | LessEqual => true,
        _ => false
    }
}

// It matches all a bintoken and try to get his "priority"
// the higher the priority, the lowest type of operations they'll get
// Multiplication only gets from Factor()
// Add gets from Multiplication that gets from Factor()

fn get_priority(token: BinKind) -> u8 {
    use BinKind::*;

    match token {
        Mul => 5,
        Div => 5,
        Add => 4,
        Sub => 4,
        And => 3,
        Or => 3,
        Xor => 3,
        Diff => 2,
        Equal => 2,
        Less => 2,
        Greater => 2,
        LessEqual => 2,
        GreaterEqual => 2,
        AndCmp => 1,
        OrCmp => 1,
    }
}

// This module matches all the binary expressions
impl<'a> Parser<'a> {
    fn factor(&mut self,accept_string: bool) -> Result<Node, CompilerError> {
        match self.next {
            Some(Token { kind, .. }) => match kind {
                TokenKind::Number => {
                    self.advance()?;
                    Ok(Node::new(NodeKind::Number, self.actual_span()))
                },
                TokenKind::Ident => {
                    self.call()
                },
                TokenKind::LPar => {
                    self.advance()?;
                    let expr = self.expr(1)?;
                    self.eat(TokenKind::RPar)?;
                    Ok(expr)
                },
                TokenKind::Str => {
                    if accept_string {
                        self.advance()?;
                        Ok(Node::new(NodeKind::Str, self.actual_span()))
                    }else{
                        Err(self.unexpected())
                    }
                },
                TokenKind::BinToken(BinKind::Sub) => {
                    self.advance()?;

                    Ok(Node::new(
                        NodeKind::Unary(Box::new(self.factor(false)?)),
                        self.actual_span()
                    ))
                },
                _ => Err(self.unexpected()),
            },
            _ => Err(self.unexpected()),
        }
    }

    pub fn expr(&mut self, priority: u8) -> Result<Node, CompilerError> {
        use TokenKind::*;

        if priority == 6 {
            return self.factor(true);
        }

        let mut left = self.expr(priority + 1)?;
        while let Some(Token { kind: BinToken(bintkn), .. }) = self.next {
            if priority != get_priority(bintkn) {
                break;
            } else {
                self.advance()?;
                let right = self.expr(priority + 1)?;
                let span = Parser::mix_span(left.span, right.span);
                left = Node::new(NodeKind::Binary(Box::new(left), bintkn, Box::new(right)), span)
            }
            if not_associative(bintkn) {
                break;
            }
        }
        Ok(left)
    }
}
