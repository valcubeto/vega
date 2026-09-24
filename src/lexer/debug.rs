use std::fmt;
// use crossterm::style::Stylize;
use super::{ Token, TokenKind };

#[cfg(debug_assertions)]
impl<'s> fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind as K;
        {
            let (start, end) = self.pos.as_span();
            write!(f, "#{}..{} ", start, end)?;
        }
        match &self.kind {
            K::Dummy => unreachable!(),
            // LITERALS (kinda)
            K::Keyword(kw) => write!(f, "Keyword[{}]({:?})", kw.len(), kw),
            K::Word(word) => write!(f, "Word[{}]({:?})", word.len(), word),
            K::RawString(string) => write!(f, "RawString[{}]({:?})", string.len(), string),
            K::String(string) => write!(f, "String[{}]({:?})", string.len(), string),
            K::Char(char) => write!(f, "Char({:?})", char),
            K::Integer(n) => write!(f, "Integer {n}"),
            K::Decimal(n) => write!(f, "Decimal {n}"),
            // PUNCTUATION
            K::NewLine => write!(f, "`\\n`"),
            K::Semicolon => write!(f, "`;`"),
            K::Comma => write!(f, "`,`"),
            // OPS
            K::Assign => write!(f, "`=`"),
            K::Equals => write!(f, "`==`"),
            K::Plus => write!(f, "`+`"),
            K::Minus => write!(f, "`-`"),
            K::Star => write!(f, "`*`"),
            K::Slash => write!(f, "`/`"),
            // PAIRS
            K::LParen => write!(f, "`(`"),
            K::RParen => write!(f, "`)`"),
            K::LBrace => write!(f, "`{{`"),
            K::RBrace => write!(f, "`}}`"),
            K::LBracket => write!(f, "`[`"),
            K::RBracket => write!(f, "`]`"),
        }
    }
}
