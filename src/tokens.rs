use crate::{
    strings::StringRegistry,
    keywords::Keyword,
    reports::{Reporter, Marker},
};
use std::{fmt, str::Chars, iter::Peekable};

pub struct Lexer<'a, 's, 'r> {
    idx: usize,
    data: &'a str,
    iter: Peekable<Chars<'a>>,
    strings: &'s StringRegistry,
    reporter: &'r mut Reporter<'a>,
}

impl<'a, 's, 'r> Lexer<'a, 's, 'r> {
    pub fn new(data: &'a str, strings: &'s StringRegistry, reporter: &'r mut Reporter<'a>) -> Self {
        Lexer {
            idx: 0,
            data,
            iter: data.chars().peekable(),
            strings,
            reporter,
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

    fn expect(&mut self, expected: char) -> Option<()> {
        match self.next() {
            Some(ch) if ch == expected => Some(()),
            _ => None
        }
    }

    pub fn lex(mut self) -> Option<Box<[Token<'s>]>> {
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
                            tokens.push(K::NewLine.at(start, self.idx));
                            break;
                        }
                    }
                    continue;
                }
                '/' => K::Slash,
                'r' if self.peek().is_some_and(|ch| ch == '"') => {
                    self.reporter.todo("raw strings", Marker::char(start));
                    return None;
                }
                'f' if self.next_if(|ch| ch == '"') => {
                    self.reporter.todo("format strings", Marker::char(start));
                    return None;
                }
                '"' => K::String(self.collect_string()?),
                ch if is_valid_word_start(ch) => {
                    let word = self.collect_word(ch);
                    match Keyword::from_str(word) {
                        Some(kw) => K::Keyword(kw),
                        None => K::Word(word)
                    }
                },
                '\'' => K::Char(self.collect_char()?),
                ch => {
                    self.reporter.syntax_soft(format!("unexpected character {ch:?}"), Marker::char(start));
                    continue;
                }
            };
            tokens.push(token.at(start, self.idx));
        }

        Some(tokens.into_boxed_slice())
    }

    fn collect_word(&mut self, ch0: char) -> &'s str {
        let start = self.idx - ch0.len_utf8();
        while self.next_if(is_valid_word_char) {}
        self.strings.push_str(&self.data[start..self.idx])
    }

    fn collect_string(&mut self) -> Option<&'s str> {
        let start = self.idx;
        let mut parsed = String::new();
        while let Some(ch) = self.next() && ch != '"' {
            match ch {
                '\n' => {
                    self.reporter.syntax("unfinished string", Marker::span(start, self.idx));
                    return None;
                }
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
        Some(self.strings.push_str(string))
    }

    fn collect_char(&mut self) -> Option<char> {
        let ch = match self.next() {
            None => {
                self.reporter.syntax("expected a character", Marker::char(self.idx));
                return None;
            }
            Some('\n') => {
                self.reporter.syntax("unfinished character", Marker::char(self.idx));
                return None;
            }
            Some('\\') => self.get_escape_sequence()?,
            Some(ch) => ch
        };
        self.expect('\'')?;
        Some(ch)
    }

    // Return escape sequence after seeing a backslash inside a string or char
    fn get_escape_sequence(&mut self) -> Option<char> {
        let start = self.idx - 1;
        match self.next() {
            None | Some('\n') => {
                self.reporter.syntax("unfinished escape sequence", Marker::span(start, self.idx));
                None
            },
            Some('\\') => Some('\\'),
            Some('e') => Some('\x1b'),
            Some('0') => Some('\0'),
            Some('n') => Some('\n'),
            Some('r') => Some('\r'),
            Some('t') => Some('\t'),
            Some('u') => {
                self.reporter.todo("unicode escape sequences", Marker::span(start, self.idx));
                None
            },
            Some('x') => {
                self.reporter.todo("hex escape sequences", Marker::span(start, self.idx));
                None
            }
            Some(ch) => {
                self.reporter.syntax(format!("unknown escape sequence \\{}", ch), Marker::span(start, self.idx));
                None
            }
        }
    }
}

pub fn is_valid_word_start(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_valid_word_char(ch: char) -> bool {
    // x'
    is_valid_word_start(ch) || ch.is_ascii_digit() || ch == '\''
}

pub struct Token<'s> {
    pub start: usize,
    pub end  : usize,
    pub kind : TokenKind<'s>
}

impl<'s> Token<'s> {
    pub fn marker(&self) -> Marker {
        Marker::span(self.start, self.end)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind<'s> {
    EoF,
    // LITERALS (kinda)
    Keyword(Keyword),
    Word(&'s str),
    RawString(&'s str),
    String(&'s str),
    Char(char),

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
        Token { start, end, kind: self }
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

impl<'s> fmt::Display for TokenKind<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind as K;
        match self {
            K::EoF => write!(f, "end of file"),
            // Literals
            K::Keyword(kw) => write!(f, "keyword `{}`", kw.as_str()),
            K::Word(word) => write!(f, "identifier `{}`", word),
            K::RawString(_) | K::String(_) => write!(f, "string"),
            K::Char(ch) => write!(f, "character {:?}", ch),
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

#[cfg(debug_assertions)]
impl<'s> fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind as K;
        write!(f, "#{:->5} ", self.start)?;
        match self.kind {
            K::EoF => write!(f, "EoF"),
            // LITERALS (kinda)
            K::Keyword(kw) => write!(f, "Keyword[{}]({:?})", kw.len(), kw),
            K::Word(word) => write!(f, "Word[{}]({:?})", word.len(), word),
            K::RawString(string) => write!(f, "RawString[{}]({:?})", string.len(), string),
            K::String(string) => write!(f, "String[{}]({:?})", string.len(), string),
            K::Char(char) => write!(f, "Char({:?})", char),
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
