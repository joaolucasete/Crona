use crate::BinKind;
use crate::CompilerError;
use crate::Node;
use crate::Parser;
use crate::TokenKind;
use crate::Token;
use super::NodeKind;
use std::boxed::Box;

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

        Equal => 2,
        Less => 2,
        Greater => 2,
        LessEqual => 2,
        GreaterEqual => 2,

        AndCmp => 1,
        OrCmp => 1,
    }
}

impl<'a> Parser<'a> {
    fn factor(&mut self) -> Result<Node, CompilerError> {
        match self.next {
            Some(Token { kind, .. }) => match kind {
                TokenKind::Number => {
                    self.advance();
                    return Ok(
                        Node::new(
                            NodeKind::Number,
                            self.actual_span()
                        )
                    )
                },
                _ => {}
            },
            _ => {}
        }
        Err(CompilerError::UnfinishedString)
    }

    pub fn expr(&mut self, priority: u8) -> Result<Node, CompilerError> {
        use TokenKind::*;

        if(priority == 6){
            return self.factor();
        } 

        let mut left = self.expr(priority + 1)?;
        while let Some(Token { kind: BinToken( bintkn ), ..}) = self.next {
            if priority != get_priority(bintkn) {
                break;
            } else{
                self.advance()?;
                let right = self.expr(priority + 1)?;
                let span = Parser::mix_span(left.span,right.span);
                left = Node::new(
                    NodeKind::Binary(Box::new(left),bintkn,Box::new(right)),
                    span
                )
            }
        }
        Ok(left)
    }
}
