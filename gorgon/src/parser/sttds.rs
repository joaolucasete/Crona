use super::NodeKind;
use crate::CompilerError;
use crate::Node;
use crate::Parser;
use crate::Token;
use crate::TokenKind;
use std::boxed::Box;

impl<'a> Parser<'a> {

    /*
     * Var matches some types of var assignments.
     * Basically, constants, resigns and incremental operators
     */

    pub fn var(&mut self, name: Node) -> Result<Node, CompilerError> {
        if let Some(Token { kind, .. }) = self.next {
            match kind {
                TokenKind::Short => {
                    self.advance()?;
                    let value = self.expr(1)?;
                    let span = Parser::mix_span(name.span, self.actual_span());
                    Ok(Node::new(NodeKind::VarDecl(false, Box::new(name), Box::new(value)), span))
                },
                operation @ (TokenKind::Assign | TokenKind::AddEqual | TokenKind::MulEqual | TokenKind::SubEqual | TokenKind::DivEqual) => {
                    self.advance()?;
                    let val = self.expr(1)?;
                    let span = Parser::mix_span(name.span, self.actual_span());
                    Ok(Node::new(NodeKind::VarSet(operation, Box::new(name), Box::new(val)), span))
                }
                _ => Ok(name),
            }
        } else {
            Err(self.unexpected())
        }
    }

    /*
     * It's the line entry point basically. It matches functions,
     * variables, if, fors, returns, breaks, in, out and continue.
     */

    pub fn statement(&mut self) -> Result<Node, CompilerError> {
        if let Some(Token { kind, .. }) = self.next {
            match kind {
                TokenKind::Ident => {
                    let name = self.expr(1)?;
                    if let Node { kind: NodeKind::Name(_), .. } = name {
                        self.var(name)
                    } else {
                        Ok(name)
                    }
                }
                TokenKind::Mut => {
                    let name = self.expr(1)?;
                    if let Node { kind: NodeKind::Name(_), .. } = name {
                        let val = self.expr(1)?;
                        let span = Parser::mix_span(name.span, self.actual_span());
                        Ok(Node::new(NodeKind::VarDecl(true, Box::new(name), Box::new(val)), span))
                    } else {
                        Err(self.unexpected())
                    }
                }
                _ => self.expr(1),
            }
        } else {
            Err(self.unexpected())
        }
    }

    /*
     * Compound gets all statements until some token
     */

    pub fn compound(&mut self, predicative: &dyn Fn(TokenKind) -> bool) -> Result<Node,CompilerError> {
        let mut sttds = Vec::new();
        let start = self.actual_span();
        while let Some(Token { kind, ..}) = self.next {
            if predicative(kind) {
                break;
            }
            sttds.push(self.statement()?);
        }
        let span = Parser::mix_span(start,self.actual_span());
        Ok(
            Node::new( NodeKind::Compound(sttds), span)
        )
    }
}
