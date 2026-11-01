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
        let mut statements: Vec<Expr> = vec![];

        while let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::NewLine | TokenKind::Semicolon) {
                let _ = self.next();
                continue;
            }

            const MIN_BP: u8 = 0;
            if let Some(stmt) = self.parse_expr_bp(MIN_BP)? {
                statements.push(stmt);
            }
            // else {
            //     None means end of file.
            // }
        }

        Ok(statements.into_boxed_slice())
    }

    /// Parse an expression with binding powers.
    pub fn parse_expr_bp(&mut self, mbp: u8) -> anyhow::Result<Option<Expr>> {
        let Some(token) = self.next() else { return Ok(None) };
        let mut lhs = match token.kind {
            TokenKind::Word(sym) => ExprKind::Ident(sym).at(token.pos),
            TokenKind::String(sym) => ExprKind::String(sym).at(token.pos),
            TokenKind::Char(ch) => ExprKind::Char(ch).at(token.pos),
            _ => todo!()
        };
        Ok(Some(lhs))
    }
}
