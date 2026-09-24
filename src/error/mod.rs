mod display;

use std::path::Path;

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

// Don't ask.
#[macro_use]
mod _macro {
    #[macro_export]
    macro_rules! here {
        () => { format!("{}:{}:{}", file!(), line!(), column!()) };
    }
}
pub use here;

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
pub struct Error<'a> {
    pub kind: ErrorKind,
    pub message: Box<str>,
    pub file: &'a Path,
    pub pos: Marker,
}

// #[allow(dead_code)]
// impl Error {
//     pub fn new(kind: ErrorKind, message: Box<str>, pos: Marker) -> Self {
//         Error { kind, message, pos }
//     }
// }

// pub fn todo(message: impl fmt::Display, pos: Marker) -> Error {
//     Error {
//         kind: ErrorKind::NotYetImplemented,
//         message: message.to_string().into_boxed_str(),
//         pos
//     }
// }

// pub fn syntax_error(message: impl fmt::Display, pos: Marker) -> Error {
//     Error {
//         kind: ErrorKind::SyntaxError,
//         message: message.to_string().into_boxed_str(),
//         pos
//     }
// }

// pub fn parse_error(message: impl fmt::Display, pos: Marker) -> Error {
//     Error {
//         kind: ErrorKind::ParseError,
//         message: message.to_string().into_boxed_str(),
//         pos
//     }
// }

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
