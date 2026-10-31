use crate::common::*;
use crate::{ keywords::Keyword, };

pub struct Token {
    pub pos: Marker,
    pub kind: TokenKind
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum TokenKind {
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

impl<'t> TokenKind {
    pub fn at(self, start: usize, end: usize) -> Token {
        Token {
            pos: Marker::Span(start, end),
            kind: self
        }
    }
}
