//! A specialized data type to hold unique strings.

use crate::common::*;
use string_interner::{ StringInterner as Interner_, DefaultBackend, DefaultSymbol };
// use inturn::Interner as _Interner;

/// An identifier for a unique string.
pub type Symbol = DefaultSymbol;

pub struct Interner {
    data: Interner_<DefaultBackend>,
}

impl Interner {
    pub fn with_capacity(capacity: usize) -> Self {
        Interner {
            data: Interner_::with_capacity(capacity)
        }
    }

    pub fn intern<S>(&mut self, string: S) -> Symbol
    where
        S: AsRef<str>
    {
        self.data.get_or_intern(string)
    }

    pub fn lookup(&self, sym: Symbol) -> &str {
        self.data.resolve(sym).expect("interned key was not present")
    }
}
