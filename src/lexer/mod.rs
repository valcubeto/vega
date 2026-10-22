mod tokens; pub use tokens::*;
mod display;
#[cfg(debug_assertions)]
mod debug;

use crate::common::*;

use crate::{
    keywords::Keyword,
};
use std::{ str::Chars, iter::Peekable, };

pub struct Lexer<'a> {
    file: &'a Path,
    source: &'a str,
    data: TextContainer,
    iter: Peekable<Chars<'a>>,
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a Path, source: &'a str) -> Self {
        Lexer {
            file,
            source,
            data: TextContainer::with_capacity(source.len() / 4 * 3),
            idx: 0,
            iter: source.chars().peekable(),
        }
    }

    pub fn lex<'t>(mut self) -> crate::vega::Result<Box<[Token<'t>]>> {
        use TokenKind as K;
        let mut tokens = Vec::new();

        while let Some(ch) = self.next() {
            let start = self.idx - ch.len_utf8();
            let token = match ch {
                ' ' | '\t' | '\r' => continue,
                '\n' => K::NewLine,
                ';' => K::Semicolon,
                ',' => K::Comma,
                '(' => K::LParen,
                ')' => K::RParen,
                '{' => K::LBrace,
                '}' => K::RBrace,
                '[' => K::LBracket,
                ']' => K::RBracket,
                '+' => K::Plus,
                '-' => K::Minus,
                '*' => K::Star,
                '=' if self.next_if(|ch| ch == '=') => K::Equals,
                '=' => K::Assign,
                '/' if self.next_if(|ch| ch == '/') => {
                    // if self.next_if(|ch| ch == '/') {
                        // TODO: doc comments
                    // }
                    while let Some(ch) = self.next() {
                        if ch == '\n' {
                            tokens.push(K::NewLine.at(self.idx - 1, self.idx));
                            break;
                        }
                    }
                    continue;
                }
                '/' => K::Slash,
                'r' if self.peek().is_some_and(|ch| ch == '"') => {
                    return Err(self.todo("raw strings", Marker::Char(start)));
                }
                'f' if self.next_if(|ch| ch == '"') => {
                    return Err(self.todo("format strings", Marker::Char(start)));
                }
                '"' => K::String(self.collect_string()?),
                ch if is_valid_word_start(ch) => {
                    let word = self.collect_word(ch);
                    match Keyword::from_str(word) {
                        Some(kw) => K::Keyword(kw),
                        None => K::Word(word)
                    }
                },
                ch if ch.is_ascii_digit() => self.collect_num()?,
                '\'' => K::Char(self.collect_char()?),
                ch => {
                    return Err(self.error(format!("unexpected character {ch:?} ({})", here!()), Marker::Char(start)));
                }
            };
            tokens.push(token.at(start, self.idx));
        }

        Ok(tokens.into_boxed_slice())
    }

    fn error(&self, message: impl fmt::Display, pos: Marker) -> vega::Error {
        vega::Error {
            kind: vega::ErrorKind::SyntaxError,
            message: message.to_string().into_boxed_str(),
            file: self.file.to_path_buf(),
            pos,
        }
    }

    fn todo(&self, message: impl fmt::Display, pos: Marker) -> vega::Error {
        vega::Error {
            kind: vega::ErrorKind::NotYetImplemented,
            message: message.to_string().into_boxed_str(),
            file: self.file.to_path_buf(),
            pos,
        }
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.iter.next()?;
        self.idx += ch.len_utf8();
        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    fn next_if(&mut self, pred: fn (char) -> bool) -> bool {
        let cond = self.peek().is_some_and(pred);
        if cond {
            let _ = self.next();
        }
        cond
    }

    fn expect(&mut self, test: fn (char) -> bool) -> vega::Result<char> {
        match self.next() {
            Some(ch) if test(ch) => Ok(ch),
            _ => Err(self.error("unexpected character", Marker::Char(self.idx)))
        }
    }

    fn collect_word(&mut self, ch0: char) -> &str {
        let start = self.idx - ch0.len_utf8();
        while self.next_if(is_valid_word_char) {}
        self.data.push_str(&self.source[start..self.idx])
    }

    fn collect_string(&mut self) -> vega::Result<&'_ str> {
        let start = self.idx;
        let mut parsed = String::new();
        while let Some(ch) = self.next() && ch != '"' {
            match ch {
                '\n' => {
                    // TODO: new lines in strings, with indents
                    return Err(self.error("unexpected new line inside string", Marker::Span(start, self.idx)));
                }
                '\\' => {
                    let ch = self.get_escape_sequence()?;
                    if parsed.capacity() == 0 {
                        parsed.push_str(&self.source[start..self.idx - 1]);
                    }
                    parsed.push(ch);
                }
                ch => {
                    if parsed.capacity() != 0 {
                        parsed.push(ch);
                    }
                }
            }
        }
        let string =
            if parsed.capacity() != 0 { parsed.as_str() }
            else { &self.source[start..self.idx - 1] };
        Ok(self.data.push_str(string))
    }

    fn collect_char(&mut self) -> vega::Result<char> {
        let ch = match self.next() {
            None => {
                return Err(self.error("expected a character", Marker::Char(self.idx)));
            }
            Some('\n') => {
                return Err(self.error("unfinished character", Marker::Char(self.idx)));
            }
            Some('\\') => self.get_escape_sequence()?,
            Some(ch) => ch
        };
        self.expect(|ch| ch == '\'')?;
        Ok(ch)
    }

    /// Return escape sequence after seeing a backslash inside a string or char
    fn get_escape_sequence(&mut self) -> vega::Result<char> {
        let start = self.idx - 1;
        match self.next() {
            None | Some('\n') => Err(self.error("unfinished escape sequence", Marker::Span(start, self.idx))),
            Some(ch @ ('\\' | '{' | '\'' | '"')) => Ok(ch),
            Some('e') => Ok('\x1b'),
            Some('0') => Ok('\0'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => {
                Err(self.todo("unicode escape sequences", Marker::Span(start, self.idx)))
            }
            Some('x') => {
                Err(self.todo("hex escape sequences", Marker::Span(start, self.idx)))
            }
            Some(ch) => Err(self.error(format!("invalid escape sequence \\{}", ch), Marker::Span(start, self.idx)))
        }
    }

    fn collect_num(&mut self) -> vega::Result<TokenKind<'_>> {
        let start = self.idx - 1;
        let mut num = String::from(&self.source[start..self.idx]);
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                let _ = self.next();
                num.push(ch);
                continue;
            } else if ch == '_' {
                if self.peek().is_none_or(|ch| !ch.is_ascii_digit()) {
                    return Err(self.error("expected digit after number separator", Marker::Char(self.idx)));
                }
                let _ = self.next();
            } else {
                break;
            }
        }
        Ok(TokenKind::Integer(self.data.push_str(num.as_str())))
    }
}

pub fn is_valid_word_start(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_valid_word_char(ch: char) -> bool {
    // x'
    is_valid_word_start(ch) || ch.is_ascii_digit() || ch == '\''
}
