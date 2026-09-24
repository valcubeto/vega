use std::fmt;
use super::{ TokenKind };

impl<'s> fmt::Display for TokenKind<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind as K;
        match self {
            K::Dummy => unreachable!(),
            // Literals
            K::Keyword(kw) => write!(f, "keyword `{}`", kw.as_str()),
            K::Word(word) => write!(f, "identifier `{}`", word),
            K::RawString(_) | K::String(_) => write!(f, "string"),
            K::Char(ch) => write!(f, "character {:?}", ch),
            K::Integer(n) => write!(f, "integer {n}"),
            K::Decimal(n) => write!(f, "decimal {n}"),
            // Punctuation
            K::NewLine => write!(f, "new line"),
            K::Semicolon => write!(f, "semicolon"),
            K::Comma => write!(f, "comma"),
            // Ops
            K::Assign => write!(f, "equals sign"),
            K::Equals => write!(f, "double equals sign"),
            K::Plus => write!(f, "plus sign"),
            K::Minus => write!(f, "dash"),
            K::Star => write!(f, "asterisk"),
            K::Slash => write!(f, "slash"),
            // Pairs
            K::LParen => write!(f, "left parenthesis"),
            K::RParen => write!(f, "right parenthesis"),
            K::LBrace => write!(f, "left brace"),
            K::RBrace => write!(f, "right brace"),
            K::LBracket => write!(f, "left bracket"),
            K::RBracket => write!(f, "right bracket"),
        }
    }
}
