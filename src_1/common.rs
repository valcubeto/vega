pub use crate::{
    strings::TextContainer,
    lexer::Lexer,
    parser::Parser,
    vega::{
        self,
        Marker,
    }
    // interpreter::{ Interpreter, Value, Runtime },
};

pub use std::{
    path::{ Path, PathBuf },
    process::exit,
    io::{ self, Write },
    fmt,
};

pub use crossterm::style::Stylize;

#[doc(hidden)]
#[macro_use]
mod _macros {
    #[macro_export]
    macro_rules! here {
        () => { format!("{}:{}:{}", file!(), line!(), column!()) };
    }
}

pub use here;
