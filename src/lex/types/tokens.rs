use crate::common::*;

use super::keywords::Keyword;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
}

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(Keyword),
    Word(Symbol),
    String(Symbol),
    Integer(Symbol),
    Decimal(Symbol, Symbol),
    Char(char),

    NewLine,
    Semicolon,

    Comma,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Hash,

    Equals,
    Assign,

    Plus,
    Hyphen,
    Star,
    Slash,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Position(
    pub u16,
    pub u16
);

impl TokenKind {
    pub fn at(self, pos: Position) -> Token {
        Token {
            kind: self,
            pos
        }
    }
}

// impl fmt::Debug for Token {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use TokenKind as Kind;
//         write!(f, "[{}:{}] ", self.pos.0, self.pos.1)?;
//         match self.kind {
//             Kind::NewLine => write!(f, "'\\n'"),
//             Kind::Hyphen  => write!(f, "'-'"),
//             Kind::Plus    => write!(f, "'+'"),
//             Kind::Word(sym) => write!(f, "Word(<symbol>)"),
//         }
//     }
// }
