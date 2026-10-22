use crate::common::*;
use crate::{ keywords::Keyword, };

pub struct Token<'t> {
    pub pos: Marker,
    pub kind: TokenKind<'t>
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum TokenKind<'t> {
    // This is to avoid that uninit bug.
    Dummy,
    // LITERALS (kinda)
    Keyword(Keyword),
    Word(&'t str),
    RawString(&'t str),
    String(&'t str),
    Char(char),
    Integer(&'t str),
    Decimal(&'t str),

    // PUNCTUATION
    NewLine,
    Semicolon,
    Comma,

    // Whatever
    Assign,

    // OPS
    Equals,
    Plus,
    Minus,
    Star,
    Slash,

    // PAIRS
    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,
}

impl<'t> TokenKind<'t> {
    pub fn at(self, start: usize, end: usize) -> Token<'t> {
        Token {
            pos: Marker::Span(start, end),
            kind: self
        }
    }
}
