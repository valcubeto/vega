use super::types::expr::{Expr, ExprKind};
use super::types::ops::{BindingPower, Op};
use crate::common::*;

const MIN_BP: u8 = 0;

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

    pub fn make_ast(mut self) -> VegaResult<Box<[Expr]>> {
        let mut statements: Vec<Expr> = vec![];

        while let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::NewLine | TokenKind::Semicolon) {
                let _ = self.next();
                continue;
            }

            if let Some(stmt) = self.parse_expr_bp(MIN_BP)? {
                statements.push(stmt);
            }
        }

        Ok(statements.into_boxed_slice())
    }

    /// Parse an expression with binding powers.
    fn parse_expr_bp(&mut self, mbp: u8) -> VegaResult<Option<Expr>> {
        // This should not be possible but anyways.
        let Some(token) = self.next() else {
            return Ok(None);
        };
        let mut lhs = match Op::prefix(token.kind) {
            Some(prefix) => {
                let value = self
                    .parse_expr_bp(prefix.bp().rbp)?
                    .context(ParseError("expected a value due to previous prefix operator"))
                    .at(token.pos)?;
                ExprKind::UnaryOp {
                    op: prefix,
                    value: Box::new(value),
                }
                .at(token.pos)
            }
            None => match token.kind {
                TokenKind::LParen => {
                    let lhs = self
                        .parse_expr_bp(MIN_BP)?
                        .context(ParseError("expected a value"))
                        .at(token.pos)?;
                    if let token = self.expect_next().at(token.pos)? && token.kind != TokenKind::RParen {
                        return None
                            .context(ParseError("expected right parenthesis"))
                            .at(token.pos);
                    }
                    lhs
                }
                TokenKind::Word(sym) => ExprKind::Ident(sym).at(token.pos),
                TokenKind::String(sym) => ExprKind::String(sym).at(token.pos),
                TokenKind::Char(ch) => ExprKind::Char(ch).at(token.pos),
                _ => None
                    .context(here!())
                    .context(ParseError(format!("unexpected {}", token.kind)))
                    .at(token.pos)?,
            },
        };

        while let Some(token) = self.peek() {
            let Some(op) = Op::infix(token.kind) else {
                return Ok(Some(lhs));
            };

            let bp = op.bp();
            if bp.lbp < mbp {
                break;
            }

            let _ = self.next();

            match op {
                op if op.is_suffix() => {
                    lhs = ExprKind::UnaryOp {
                        op,
                        value: Box::new(lhs),
                    }
                    .at(token.pos);
                }
                Op::Call => {
                    None.context(Todo("calls")).at(token.pos)?;
                }
                _ => {
                    let rhs = self
                        .parse_expr_bp(bp.rbp)?
                        .context(ParseError("expected a value due to previous operator"))
                        .at(token.pos)?;
                    lhs = ExprKind::BinaryOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                    .at(token.pos);
                }
            }
        }

        Ok(Some(lhs))
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().copied().copied()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next().copied()
    }

    fn expect_next(&mut self) -> anyhow::Result<Token> {
        self.next().context(ParseError("unexpected end of input"))
    }
}
