use crate::{strings::StringRegistry, keywords::Keyword};
use std::{fmt, str::Chars, iter::Peekable};

#[derive(Debug)]
pub struct SyntaxError(#[allow(dead_code)] String);

pub struct Lexer<'a> {
    idx: usize,
    data: &'a str,
    iter: Peekable<Chars<'a>>,
    strings: &'a StringRegistry,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str, strings: &'a StringRegistry) -> Self {
        Lexer {
            idx: 0,
            data,
            iter: data.chars().peekable(),
            strings
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

    pub fn lex(&mut self) -> Result<Vec<Token<'_>>, SyntaxError> {
        let mut tokens = Vec::new();

        #[allow(clippy::while_let_on_iterator)]
        while let Some(ch) = self.next() {
            let start = self.idx - ch.len_utf8();
            let token = match ch {
                ' ' | '\t' | '\r' => continue,
                '\n' => TokenKind::NewLine,
                ';' => TokenKind::Semicolon,
                ',' => TokenKind::Comma,
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '/' if self.next_if(|ch| ch == '/') => {
                    if self.next_if(|ch| ch == '/') {
                        // TODO: doc comments
                    }
                    while let Some(ch) = self.next() {
                        if ch == '\n' {
                            tokens.push(TokenKind::NewLine.at(self.idx - 1));
                            break;
                        }
                    }
                    continue;
                }
                '"' => TokenKind::String(self.collect_string()?),
                '#' if self.next_if(|ch| ch == '"') => {
                    todo!("raw strings");
                }
                '$' if self.next_if(|ch| ch == '"') => {
                    panic!("format strings");
                }
                ch if is_valid_word_start(ch) => {
                    let word = self.collect_word(ch);
                    match Keyword::from_str(word) {
                        Some(kw) => TokenKind::Keyword(kw),
                        None => TokenKind::Word(word)
                    }
                },
                ch => return Err(SyntaxError(format!("unexpected token {ch:?}")))
            };
            tokens.push(token.at(start));
        }

        Ok(tokens)
    }

    pub fn collect_word(&mut self, ch0: char) -> &'a str {
        let start = self.idx - ch0.len_utf8();
        while self.next_if(is_valid_word_char) {}
        self.strings.push_str(&self.data[start..self.idx])
    }

    pub fn collect_string(&mut self) -> Result<&'a str, SyntaxError> {
        let start = self.idx;
        let mut parsed = String::new();
        while let Some(ch) = self.next() && ch != '"' {
            match ch {
                '\n' => return Err(SyntaxError("unfinished string".to_owned())),
                '\\' => {
                    let ch = self.get_escape_sequence()?;
                    if parsed.capacity() == 0 {
                        parsed.push_str(&self.data[start..self.idx - 1]);
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
            else { &self.data[start..self.idx - 1] };
        Ok(self.strings.push_str(string))
    }

    pub fn get_escape_sequence(&mut self) -> Result<char, SyntaxError> {
        Ok(match self.next() {
            None | Some('\n') => return Err(SyntaxError("unfinished escape sequence".to_owned())),
            Some('\\') => '\\',
            Some('e') => '\x1b',
            Some('0') => '\0',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('u') => return Err(SyntaxError("TODO: unicode escape sequences".to_owned())),
            Some('x') => return Err(SyntaxError("TODO: hex escape sequences".to_owned())),
            Some(ch) => return Err(SyntaxError(format!("unknown escape sequence \\{}", ch)))
        })
    }
}

pub fn is_valid_word_start(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_valid_word_char(ch: char) -> bool {
    // x'
    is_valid_word_start(ch) || ch.is_ascii_digit() || ch == '\''
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TokenKind<'a> {
    Keyword(Keyword),
    Word(&'a str),
    RawString(&'a str),
    String(&'a str),

    // PUNCTUATION
    NewLine,
    Semicolon,
    Comma,

    // PAIRS
    LParen,
    RParen,
}

impl<'a> TokenKind<'a> {
    pub fn at(self, idx: usize) -> Token<'a> {
        Token { idx, kind: self }
    }
}

pub struct Token<'a> {
    pub idx: usize,
    pub kind: TokenKind<'a>
}

impl<'a> fmt::Debug for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind as K;
        write!(f, "#{:<04} ", self.idx)?;
        match self.kind {
            K::Keyword(kw) => write!(f, "Word[{}]({:?})", kw.len(), kw),
            K::Word(word) => write!(f, "Word[{}]({:?})", word.len(), word),
            K::RawString(string) => write!(f, "RawString[{}]({:?})", string.len(), string),
            K::String(string) => write!(f, "String[{}]({:?})", string.len(), string),
            K::NewLine => write!(f, "NewLine"),
            K::Semicolon => write!(f, "Semicolon"),
            K::Comma => write!(f, "Comma"),
            K::LParen => write!(f, "LParen"),
            K::RParen => write!(f, "RParen"),
        }
    }
}
