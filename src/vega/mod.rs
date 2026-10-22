use crate::common::*;

mod display;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotYetImplemented,
    SyntaxError,
    ParseError,
    EvalError,
}

#[allow(dead_code)]
impl ErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotYetImplemented => "Not yet implemented",
            Self::SyntaxError => "Syntax error",
            Self::ParseError => "Parse error",
            Self::EvalError => "Eval error",
        }
    }
}

#[allow(dead_code)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: Box<str>,
    // I prefer having an unused capacity rather than writing <'a> all over the workspace.
    pub file: PathBuf,
    pub pos: Marker,
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub enum Marker {
    Char(usize),
    Span(usize, usize)
}

impl Marker {
    pub fn as_span(self) -> (usize, usize) {
        match self {
            Self::Char(idx) => (idx, idx + 1),
            Self::Span(start, end) => (start, end)
        }
    }
}
