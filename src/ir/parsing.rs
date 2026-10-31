use crate::common::*;
use super::types::expr::{ Expr, ExprKind };

pub struct Parser<'a> {
    input: &'a Input,
    tokens: Peekable<Iter<'a, Token>>,
    idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Input, tokens: &'a [Token]) -> Self {
        Parser {
            input,
            tokens: tokens.iter().peekable(),
            idx: 0,
        }
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().copied().copied()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next().copied()
    }

    pub fn make_ast(mut self) -> anyhow::Result<Box<[Expr]>> {
        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            if token.kind == TokenKind::NewLine {
                let _ = self.next();
                continue;
            }
            if let Some(stmt) = self.parse_expr()? {
                statements.push(stmt);
            }
        }

        Ok(statements.into_boxed_slice())
    }

    pub fn parse_expr(&mut self) -> anyhow::Result<Option<Expr>> {
        let Some(token) = self.next() else { return Ok(None) };
        let mut expr = match token.kind {
            TokenKind::String(sym) => {
                Expr {
                    kind: ExprKind::String(sym)
                }
            }
            _ => todo!()
        };
        Ok(Some(expr))
    }
}
