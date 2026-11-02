#[allow(unused_imports)]
pub use {
    crate::{
        lex::types::tokens::*,
        text::interner::{Interner, Symbol},
        errors::*,
    },
    anyhow::Context,
    crossterm::style::Stylize,
    hashbrown::HashMap,
    std::{
        fmt,
        io::{self, Write},
        iter::Peekable,
        path::{Path, PathBuf},
        process::exit,
        slice::Iter,
        str::Chars,
    },
};

pub struct Input {
    pub source: Box<Path>,
    pub data: Box<str>,
}

#[macro_use]
mod _macros {
    #[macro_export]
    macro_rules! here {
        () => {
            format!("    at {}:{}:{}", file!(), line!(), column!())
        };
    }

    #[macro_export]
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            ::hashbrown::HashMap::from([
                $( ($key.to_owned(), $value) ),*
            ])
        };
    }
}

pub use { hashmap, here };
