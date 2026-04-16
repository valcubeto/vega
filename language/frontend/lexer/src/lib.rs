use prelude::types::*;
#[allow(unused_imports)]
use prelude::terminal::*;
use syntax::keywords::Keyword;
use syntax::operators::Operator;

pub enum Token {
    NewLine,
    Comma,
    Semicolon,
    Colon,
    OpeningBrace,
    ClosingBrace,
    OpeningParen,
    ClosingParen,
    OpeningBracket,
    ClosingBracket,
    /// `->`
    ThinArrow,
    /// `=>`
    FatArrow,
    /// `@`
    DecoratorStart,
    DecoratorEnd,
    /// `#[`
    MetadataStart,
    /// `]`
    MetadataEnd,
    /// `$`
    MacroVarStart,
    /// `|`
    LambdaStart,
    /// `|`
    LambdaEnd,
    Operator(Operator),
    Keyword(Keyword),
    Identifier(Str),
    MacroName(Str),
}

impl Token {}
