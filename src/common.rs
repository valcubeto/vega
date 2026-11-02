#[allow(unused_imports)]
pub use {
    std::{
        io::{ self, Write },
        fmt,
        path::{ Path, PathBuf },
        iter::Peekable,
        str::Chars,
        slice::Iter,
        process::exit,
    },
    crossterm::style::Stylize,
    hashbrown::HashMap,
    anyhow::Context,
    crate::{
        text::interner::{ Interner, Symbol },
        lex::types::tokens::*,
    },
};

pub type VegaResult<T> = Result<T, (Position, anyhow::Error)>;

pub trait VegaMoreContext<T> {
    fn at(self, pos: Position) -> VegaResult<T>;
}

impl<T> VegaMoreContext<T> for anyhow::Result<T> {
    fn at(self, pos: Position) -> VegaResult<T> {
        self.map_err(|ctx| (pos, ctx))
    }
}

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

    #[macro_export]
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            ::hashbrown::HashMap::from([
                $( ($key.to_owned(), $value) ),*
            ])
        };
    }
}

pub use { here, hashmap };
