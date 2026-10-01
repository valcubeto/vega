use crate::{ keywords::Keyword, error::Marker };

pub struct Token<'rt> {
    pub pos: Marker,
    pub kind: TokenKind<'rt>
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum TokenKind<'rt> {
    // This is to avoid that uninit bug.
    Dummy,
    // LITERALS (kinda)
    Keyword(Keyword),
    Word(&'rt str),
    RawString(&'rt str),
    String(&'rt str),
    Char(char),
    Integer(&'rt str),
    Decimal(&'rt str),

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

impl<'rt> TokenKind<'rt> {
    pub fn at(self, start: usize, end: usize) -> Token<'rt> {
        Token { pos: Marker::Span(start, end), kind: self }
    }
}
