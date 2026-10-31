#[allow(unused_imports)]
pub use {
    std::{
        io::{ self, Write },
        fmt,
        path::{ Path, PathBuf },
        iter::Peekable,
        str::Chars,
        slice::Iter,
    },
    crossterm::style::Stylize,
    hashbrown::HashMap,
    anyhow::Context,
    crate::{
        text::interner::{ Interner, Symbol },
        lex::types::tokens::*,
    },
};

pub struct Input {
    pub source: Box<Path>,
    pub data: Box<str>,
}

#[doc(hidden)]
#[macro_use]
mod _macros {
    #[macro_export]
    macro_rules! here {
        () => { format!("{}:{}:{}", file!(), line!(), column!()) };
    }
}

pub use here;
