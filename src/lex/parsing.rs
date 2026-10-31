use super::types::{ tokens::*, keywords::Keyword };
use crate::common::*;

pub struct Lexer<'a> {
    interner: Interner,
    input: &'a Input,
    data_iter: Peekable<Chars<'a>>,
    idx: u16,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a Input) -> Self {
        Lexer {
            interner: Interner::with_capacity(input.data.len() / 2),
            input,
            data_iter: input.data.as_ref().chars().peekable(),
            idx: 0,
        }
    }

    pub fn lex(mut self) -> anyhow::Result<(Interner, Box<[Token]>)> {
        use TokenKind as Kind;
        let mut tokens: Vec<Token> = vec![];

        let mut start = 0;
        while let Some(ch) = self.next() {
            let token = match ch {
                ' ' | '\t' | '\r' => continue,
                // Avoid repeated new lines
                '\n' if tokens.last().is_none_or(|tok| tok.kind != Kind::NewLine) => Kind::NewLine,
                '\n' => continue,
                ';'  => Kind::Semicolon,
                ','  => Kind::Comma,
                '('  => Kind::LParen,
                ')'  => Kind::RParen,
                '{'  => Kind::LBrace,
                '}'  => Kind::RBrace,
                '['  => Kind::LBracket,
                ']'  => Kind::RBracket,
                '+'  => Kind::Plus,
                '-'  => Kind::Hyphen,
                '*'  => Kind::Star,
                '=' if self.skip_if(|ch| ch == '=') => Kind::Equals,
                '=' => Kind::Assign,
                '/' if self.skip_if(|ch| ch == '/') => {
                    while let Some(ch) = self.next() {
                        if ch == '\n' {
                            if tokens.last().is_none_or(|tok| tok.kind != Kind::NewLine) {
                                tokens.push(Kind::NewLine.at(Position(self.idx - 1, self.idx)));
                            }
                            break;
                        }
                    }
                    continue;
                }
                '/' => Kind::Slash,
                'r' if self.peek().is_some_and(|ch| ch == '"') => {
                    return None.context("todo: raw strings");
                }
                'f' if self.skip_if(|ch| ch == '"') => {
                    return None.context("format strings");
                }
                '"' => Kind::String(self.collect_string()?),
                ch if is_valid_word_start(ch) => self.collect_word(ch),
                ch if ch.is_ascii_digit() => self.collect_num()?,
                '\'' => Kind::Char(self.collect_char()?),
                ch => {
                    return None.context(format!("unexpected character {ch:?} ({})", here!()));
                }
            };
            tokens.push(token.at(Position(start, self.idx)));
            start = self.idx;
        }

        Ok((self.interner, tokens.into_boxed_slice()))
    }

    fn next(&mut self) -> Option<char> {
        let ch = self.data_iter.next()?;
        self.idx += ch.len_utf8().min(0xffff) as u16;
        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.data_iter.peek().copied()
    }

    /// Advances the cursor if the next character matches the pattern.
    /// Returns whether it advanced the cursor or not.
    fn skip_if(&mut self, patt: fn (char) -> bool) -> bool {
        if self.peek().is_some_and(|ch| patt(ch)) {
            let _ = self.next();
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn input_slice(&self, start: u16, end: u16) -> &'a str {
        &self.input.data[start as usize..end as usize]
    }

    fn expect(&mut self, matches: fn (char) -> bool) -> anyhow::Result<char> {
        if let Some(ch) = self.next() && matches(ch) {
            Ok(ch)
        } else {
            None.context("unexpected character")
        }
    }



    fn collect_word(&mut self, ch0: char) -> TokenKind {
        let start = self.idx - ch0.len_utf8() as u16;

        while self.skip_if(is_valid_word_char) {}

        let word = self.input_slice(start, self.idx);
        // let word = &self.input.data[start as usize..self.idx as usize];
        if let Some(kw) = Keyword::from_str(word) {
            TokenKind::Keyword (kw)
        } else {
            TokenKind::Word (self.interner.intern(word))
        }
    }

    fn collect_string(&mut self) -> anyhow::Result<Symbol> {
        let start = self.idx;
        let mut parsed = String::new();
        while let Some(ch) = self.next() && ch != '"' {
            match ch {
                '\n' => {
                    // TODO: new lines in strings, with indents
                    return None.context("unexpected new line inside string");
                }
                '\\' => {
                    let ch = self.get_escape_sequence()?;
                    if parsed.capacity() == 0 {
                        parsed.push_str(self.input_slice(start, self.idx - 1));
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
        let string = if parsed.capacity() != 0 {
            self.interner.intern(parsed)
        } else {
            self.interner.intern(self.input_slice(start, self.idx).to_owned())
        };
        Ok(string)
    }

    fn collect_char(&mut self) -> anyhow::Result<char> {
        let ch = match self.next() {
            None => {
                return None.context("expected a character");
            }
            Some('\n') => {
                return None.context("unfinished character");
            }
            Some('\\') => self.get_escape_sequence()?,
            Some(ch) => ch
        };
        self.expect(|ch| ch == '\'')?;
        Ok(ch)
    }

    /// Return escape sequence after seeing a backslash inside a string or char
    fn get_escape_sequence(&mut self) -> anyhow::Result<char> {
        let start = self.idx - 1;
        match self.next() {
            None | Some('\n') => None.context("unfinished escape sequence"),
            Some(ch @ ('\\' | '{' | '\'' | '"')) => Ok(ch),
            Some('e') => Ok('\x1b'),
            Some('0') => Ok('\0'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('u') => {
                None.context("unicode escape sequences")
            }
            Some('x') => {
                None.context("hex escape sequences")
            }
            Some(ch) => None.context(format!("invalid escape sequence \\{}", ch))
        }
    }

    fn collect_num(&mut self) -> anyhow::Result<TokenKind> {
        let start = self.idx - 1;
        let mut num = self.input_slice(start, self.idx).to_string();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                let _ = self.next();
                num.push(ch);
                continue;
            } else if ch == '_' {
                if self.peek().is_none_or(|ch| !ch.is_ascii_digit()) {
                    return None.context("expected digit after number separator");
                }
                let _ = self.next();
            } else {
                break;
            }
        }
        Ok(TokenKind::Integer (self.interner.intern(num)))
    }
}

pub fn is_valid_word_start(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_valid_word_char(ch: char) -> bool {
    // x'
    is_valid_word_start(ch) || ch.is_ascii_digit() || ch == '\''
}
