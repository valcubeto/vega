// Note: should I move prelude to its own sub-crate?
#![allow(unused)]
pub type Text = Box<str>;
pub type Array<T> = Box<[T]>;

// pub struct Identifier(pub Text);
