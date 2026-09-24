use crate::{ keywords::Keyword, error::Marker };

pub struct Token<'s> {
    pub pos: Marker,
    pub kind: TokenKind<'s>
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum TokenKind<'s> {
    // This is to avoid that uninit bug.
    Dummy,
    // LITERALS (kinda)
    Keyword(Keyword),
    Word(&'s str),
    RawString(&'s str),
    String(&'s str),
    Char(char),
    Integer(Box<str>),
    Decimal(Box<str>),

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

impl<'s> TokenKind<'s> {
    pub fn at(self, start: usize, end: usize) -> Token<'s> {
        Token { pos: Marker::Span(start, end), kind: self }
    }

    // Get binding PAWA!!
    // pub fn bp(self) -> Option<BindingPower> {
    //     let (lbp, rbp) = match self {
    //         Self::LParen | Self::LBracket | Self::LBrace => (100, 101),
    //         // Not every token has PAWA...
    //         _ => return None
    //     };
    //     Some(BindingPower { lbp, rbp })
    // }
}
