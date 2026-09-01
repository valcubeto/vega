use crate::{tokens::{Token, TokenKind}, strings::StringRegistry};
use std::{path::Path, iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    file: &'a Path,
    strings: &'a StringRegistry,
    tokens: Peekable<Iter<'a, Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a Path, strings: &'a StringRegistry, tokens: &'a [Token<'a>]) -> Self {
        Parser {
            file,
            strings,
            tokens: tokens.iter().peekable(),
        }
    }

    fn next(&mut self) -> Option<&'a Token<'a>> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&&'a Token<'a>> {
        self.tokens.peek()
    }

    fn next_if(&mut self, pred: fn (&'a Token<'a>) -> bool) -> bool {
        let cond = self.peek().is_some_and(|token| pred(*token));
        if cond {
            let _ = self.next();
        }
        cond
    }

    pub fn parse(&mut self) -> Result<Vec<Expr<'a>>, ParseError> {
        use TokenKind as K;
        let mut ast = Vec::new();
        while let Some(mut lhs) = self.get_primary()? {
            while let Some(token) = self.peek() {
                match token.kind {
                    K::LParen => {
                        let args = self.collect_call_args()?;
                        lhs = Expr::Call { lhs: Box::new(lhs), args }
                    }
                    _ => break
                }
            }
            ast.push(lhs);
        }
        Ok(ast)
    }

    fn get_primary(&mut self) -> Result<Option<Expr<'a>>, ParseError> {
        use TokenKind as K;
        while self.next_if(|token| matches!(token.kind, K::NewLine | K::Semicolon)) {}
        let Some(token) = self.next() else {
            return Ok(None);
        };
        Ok(Some(match token.kind {
            K::Word(word) => Expr::Ident(word),
            K::String(string) => Expr::String(string),
            _ => return Err(ParseError(format!("unexpected token {token:?}")))
        }))
    }

    fn collect_call_args(&mut self) -> Result<Vec<Expr<'a>>, ParseError> {
        let mut args = Vec::new();
        use TokenKind as K;
        while let Some(token) = self.next() {
            match token.kind {
                K::NewLine => continue,
                K::RParen => return Ok(args),
                _ => self.parse_expr()
            }
        }
        return Err(ParseError("unclosed argument list, expected ')'".to_owned()))
    }
}

#[derive(Debug)]
pub struct ParseError(String);

pub enum Expr<'a> {
    Ident(&'a str),
    String(&'a str),
    Call {
        lhs: Box<Expr<'a>>,
        args: Vec<Expr<'a>>
    }
}
