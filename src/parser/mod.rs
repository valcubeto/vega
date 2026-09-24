mod ops; pub use ops::*;
mod expr; pub use expr::*;

use crate::{ lexer::{ Token, TokenKind }, error::* };
use std::{ iter::Peekable, slice::Iter, path::Path, fmt };

#[allow(dead_code)]
pub struct Parser<'a, 't, 's> {
    file: &'a Path,
    data: &'a str,
    tokens: Peekable<Iter<'t, Token<'s>>>,
    current: &'t Token<'s>,
}

/*
 * ast = expr*
 * prefix = `not` | `-`
 * suffix = `?` | `!` | `(` expr `)`
 * infix  = `+` | `-`
 * expr = [prefix]* value [suffix]* [infix expr]*
 *
 * make_ast():
 *     ast = []
 *     while tokens:
 *         ast.push(self.parse_node(0, 0)?)
 *     return ast
 *
 * parse_node(lforce, rforce):
 *     primary = match token:
 *         on newline | semicolon: continue
 *         on prefix: return expr::<prefix>(self.parse_expr())
 *         on literal: assign
 *         on lparen: self.parse_tuple()
 *         on lbracket: self.parse_array()
 *         on pipe: self.parse_lambda()
 *         else: return "unexpected token {token}"
 *     while tokens:
 *         match token:
 *             on newline: continue
 *             on suffix: primary = expr::<suffix>(&primary)
 *             on infix:
 *                 (lforce, rforce) = infix.force
 *                 whatever
 *             else: break
 *     return primary
 *
 */

#[allow(dead_code)]
impl<'a, 't, 's> Parser<'a, 't, 's> {
    pub fn new(file: &'a Path, data: &'a str, tokens: &'t [Token<'s>]) -> Self {
        Parser {
            file,
            data,
            tokens: tokens.iter().peekable(),
            // next() or peek() is immediatly called.
            current: &Token { kind:TokenKind::Dummy, pos: Marker::Span(0, 0) },
        }
    }

    #[must_use]
    fn next(&mut self) -> Option<&'t Token<'s>> {
        self.tokens.next().inspect(|&t| self.current = t)
    }

    fn expect_some(&mut self) -> Result<'a, &'t Token<'s>> {
        // Is there any Option method to do this?
        match self.tokens.next() {
            None => {
                let i = self.data.len() - self.data.chars().next_back().unwrap_or('\0').len_utf8();
                Err(self.error("unexpected end of input", Marker::Char(i)))
            }
            Some(t) => Ok(t),
        }
    }

    fn error(&self, message: impl fmt::Display, pos: Marker) -> Error<'a> {
        Error {
            kind: ErrorKind::ParseError,
            message: message.to_string().into_boxed_str(),
            file: self.file,
            pos,
        }
    }

    fn todo(&self, message: impl fmt::Display, pos: Marker) -> Error<'a> {
        Error {
            kind: ErrorKind::NotYetImplemented,
            message: message.to_string().into_boxed_str(),
            file: self.file,
            pos,
        }
    }

    #[must_use]
    fn peek(&mut self) -> Option<&'t Token<'s>> {
        self.tokens.peek().copied().inspect(|&t| self.current = t)
    }

    fn next_if(&mut self, pred: fn (&'t Token<'s>) -> bool) -> bool {
        let matched = self.peek().is_some_and(pred);
        if matched { let _ = self.next(); }
        matched
    }

    // /// End parsing tokens in a global scope.
    // pub fn consume_all(&mut self) {
    //     while let Some(token) = self.next() {
    //         if !matches!(token.kind, TokenKind::NewLine | TokenKind::Semicolon) {
    //             self.reporter.parse(format!("unexpected {token}"), Marker::span(token.idx, token.kind.len()));
    //         }
    //     }
    // }

    pub fn parse_global_scope<'h>(mut self) -> Result<'a, Box<[Expr<'s, 'h>]>> {
        use TokenKind as K;
        let mut ast = Vec::new();
        while let Some(token) = self.peek() {
            if token.kind == K::NewLine || token.kind == K::Semicolon {
                let _ = self.next();
                continue;
            }
            let expr = self.parse_expr_bp(0, true)?;
            ast.push(expr);
        }
        Ok(ast.into_boxed_slice())
    }

    pub fn parse_expr_bp(&mut self, mbp: u8, nl_check: bool) -> Result<'a, Expr<'s>> {
        use TokenKind as K;
        let token = self.expect_some()?;
        let mut lhs = match Op::prefix(&token.kind) {
            Some(prefix) => {
                Expr {
                    kind: ExprKind::UnaryOp {
                        op: prefix,
                        value: Box::new(self.parse_expr_bp(prefix.bp().rbp, nl_check)?)
                    }
                }
            }
            None => match token.kind {
                K::LParen => {
                    let lhs = self.parse_expr_bp(0, nl_check)?;
                    if let token = self.expect_some()? && token.kind != K::RParen {
                        return Err(self.error(format!("unexpected {}", token.kind), token.pos));
                    }
                    lhs
                }
                K::Word(word) => Expr { kind: ExprKind::Ident(word) },
                K::String(string) => Expr { kind: ExprKind::String(string) },
                K::Integer(ref n) => {
                    Expr { kind: ExprKind::Integer(n.parse().map_err(|error| self.error(format!("failed to parse integer: {error}"), token.pos))?) }
                }
                _ => return Err(self.error(format!("unexpected {} ({})", token.kind, here!()), token.pos))
            }
        };

        while let Some(token) = self.peek() {
            let Some(op) = Op::infix(&token.kind) else {
                return Ok(lhs)
            };

            let bp = op.bp();
            if bp.lbp < mbp {
                break;
            }

            let _ = self.next();

            match op {
                op if op.is_suffix() => {
                    lhs = Expr {
                        kind: ExprKind::UnaryOp {
                            op,
                            value: Box::new(lhs)
                        }
                    };
                }
                Op::Call => {
                    todo!("call")
                }
                _ => {
                    let rhs = self.parse_expr_bp(bp.rbp, nl_check)?;
                    lhs = Expr {
                        kind: ExprKind::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        }
                    };
                }
            }
        }

        Ok(lhs)
    }

    // fn parse_value(&mut self) -> Option<Expr<'s>> {
    //     None
    // }

    // fn collect_comma_separated(&mut self) -> Result<'a, Box<[Expr<'s>]>> {
    //     let mut args = Vec::new();
    //     // use TokenKind as K;
    //     // while let Some(token) = self.next() {
    //     //     match token.kind {
    //     //         K::NewLine => continue,
    //     //         K::RParen => return Ok(args),
    //     //         _ => self.parse_expr()?
    //     //     }
    //     // }
    //     // return Err(ParseError("unclosed argument list, expected ')'".to_owned()))
    //     Ok(args.into_boxed_slice())
    // }
}
