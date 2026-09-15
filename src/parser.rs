use crate::{
    keywords::Keyword,
    tokens::{Token, TokenKind},
    // strings::StringRegistry,
    reports::{Reporter, Marker},
};
use std::{iter::Peekable, slice::Iter, fmt};

#[allow(dead_code)]
pub struct Parser<'a, 's, 'r> {
    tokens: Peekable<Iter<'a, Token<'s>>>,
    // strings: &'s StringRegistry,
    reporter: &'r mut Reporter<'a>,
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


impl<'a, 's, 'r> Parser<'a, 's, 'r> {
    pub fn new(tokens: &'a [Token<'s>], /* strings: &'s StringRegistry, */ reporter: &'r mut Reporter<'a>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
            // strings,
            reporter,
        }
    }

    #[must_use]
    fn next(&mut self) -> Option<&'a Token<'s>> {
        self.tokens.next()
    }

    #[must_use]
    fn expect_some(&mut self) -> Option<&'a Token<'s>> {
        // Is there any Option method to do this?
        match self.tokens.next() {
            Some(t) if t.kind == TokenKind::EoF => {
                self.reporter.parse(format!("unexpected {}", t.kind), t.marker());
                None
            }
            Some(t) => Some(t),
            None => None
        }
    }

    #[must_use]
    fn peek(&mut self) -> Option<&'a Token<'s>> {
        self.tokens.peek().copied()
    }

    fn next_if(&mut self, pred: fn (&'a Token<'s>) -> bool) -> bool {
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

    pub fn parse_mod(mut self) -> Option<Box<[Expr<'s>]>> {
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
        Some(ast.into_boxed_slice())
    }

    fn parse_expr_bp(&mut self, mbp: u8, nl_breaks: bool) -> Option<Expr<'s>> {
        use TokenKind as K;
        let token = self.expect_some()?;
        let mut lhs = match Op::prefix(&token.kind) {
            Some(prefix) => {
                Expr::UnaryOp {
                    op: prefix,
                    value: Box::new(self.parse_expr_bp(prefix.bp().rbp, nl_breaks)?)
                }
            }
            None => match token.kind {
                K::LParen => {
                    let lhs = self.parse_expr_bp(0, nl_breaks)?;
                    if let token = self.expect_some()? && token.kind != K::RParen {
                        self.reporter.parse(format!("unexpected {}", token.kind), token.marker());
                        return None;
                    }
                    lhs
                }
                K::Word(word) => Expr::Ident(word),
                K::String(string) => Expr::String(string),
                _ => {
                    self.reporter.parse(format!("unexpected {}", token.kind), token.marker());
                    return None;
                }
            }
        };

        while let Some(token) = self.peek() {
            let Some(op) = Op::infix(&token.kind) else {
                return Some(lhs)
            };

            let bp = op.bp();
            if mbp > bp.lbp {
                break;
            }

            let _ = self.next();

            match op {
                Op::Not => {
                    lhs = Expr::UnaryOp {
                        op,
                        value: Box::new(lhs)
                    };
                }
                Op::Call => {}
                _ => {
                    let rhs = self.parse_expr_bp(bp.rbp, nl_breaks)?;
                    lhs = Expr::BinaryOp {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            }
        }

        Some(lhs)
    }

    // fn parse_value(&mut self) -> Option<Expr<'s>> {
    //     None
    // }

    fn collect_comma_separated(&mut self) -> Option<Box<[Expr<'s>]>> {
        let mut args = Vec::new();
        // use TokenKind as K;
        // while let Some(token) = self.next() {
        //     match token.kind {
        //         K::NewLine => continue,
        //         K::RParen => return Ok(args),
        //         _ => self.parse_expr()?
        //     }
        // }
        // return Err(ParseError("unclosed argument list, expected ')'".to_owned()))
        Some(args.into_boxed_slice())
    }
}

#[allow(dead_code)]
pub enum Expr<'s> {
    Ident(&'s str),
    String(&'s str),
    UnaryOp {
        op: Op,
        value: Box<Expr<'s>>
    },
    BinaryOp {
        op: Op,
        lhs: Box<Expr<'s>>,
        rhs: Box<Expr<'s>>,
    },
    Call {
        lhs: Box<Expr<'s>>,
        // TODO: change this to an array
        args: Box<[Expr<'s>]>,
    }
}

#[cfg(debug_assertions)]
impl<'s> fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::String(string) => write!(f, "{:?}", string),
            Expr::UnaryOp { op, value } => write!(f, "({:?} {:?})", op, value),
            Expr::BinaryOp { op, lhs, rhs } => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
            Expr::Call { lhs, args } => write!(f, "({:?} ({:?}))", lhs, args)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Op {
    Neg,
    Not,
    Add, Sub,
    Mul, Div,
    Mod,
    Pow,
    /// `x(y, z)`
    Call,
}

impl Op {
    pub fn prefix<'s>(token: &TokenKind<'s>) -> Option<Op> {
        use TokenKind as K;
        use Keyword as Kw;
        match token {
            K::Keyword(Kw::Not) => Some(Op::Not),
            _ => None
        }
    }

    pub fn infix<'s>(token: &TokenKind<'s>) -> Option<Op> {
        use TokenKind as K;
        // use Keyword as Kw;
        match token {
            K::Plus => Some(Op::Add),
            K::Minus => Some(Op::Sub),
            K::Star => Some(Op::Mul),
            K::Slash => Some(Op::Div),
            _ => None
        }
    }

    // pub fn is_suffix(self) -> bool {
    //     matches!(self, Op::Not)
    // }

    /* not x::y.z() + 5 * 3 ** 9 */
    pub fn bp(self) -> BindingPower {
        // Greater rbp means expressions are grouped LTR ((x + y) + z)
        // Greater lbp means expressions are grouper RTL (x = (y = z))
        // lbp < rbp = break
        let (lbp, rbp) = match self {
            Op::Add | Op::Sub => (50, 51),
            Op::Mul | Op::Div => (60, 61),
            Op::Mod => (65, 66),
            Op::Pow => (70, 71),
            Op::Neg | Op::Not => (0, 80),
            Op::Call => (101, 100),
            // Op::GetProp | Op::GetItem => (111, 110),
        };
        BindingPower { lbp, rbp }
    }
}


#[cfg(debug_assertions)]
impl<'s> fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Op::Not => "not",
            Op::Add => "+",
            Op::Sub | Op::Neg => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::Pow => "**",
            Op::Call => "call",
        };
        write!(f, "{}", crossterm::style::Stylize::magenta(repr))
    }
}

/// Binding PAWA!!
pub struct BindingPower {
    pub lbp: u8,
    pub rbp: u8,
}
