
mod flags2;
pub mod options;
pub mod flags;
use prelude::types::*;
use std::collections::HashSet;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParsedArgs {
    pub subcommand: Option<Str>,
    pub flags: Slice<flags::Flag>,
    pub rest: Slice<Str>,
}

macro_rules! hashset {
    ($( $key:expr ),* $(,)?) => {{
        std::collections::HashSet::from([
            $( $key ),*
        ])
    }};
}

impl ParsedArgs {
    pub fn parse() -> Self {
        let value_flags = hashset! {
            "main"
        };
        let mut flags = vec![];
        let mut args = std::env::args()
            .skip(1)              // Ignore exe path.
            .take(256);           // Just in case.
        let subcommand = args.next()
            .map(|string| string.into_boxed_str());
        ParsedArgs {
            subcommand,
            flags: flags.into(),
            rest: args
                .map(|string| string.into_boxed_str())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}
