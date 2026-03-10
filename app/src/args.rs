//! # Worker: ArgParser
//! ### Here
//! - Command line arguments
//! - Global flags
//! - Sub-command flags
//!
//! join args with \0 so its easier to parse

#[cfg(debug_assertions)]
use std::fmt;
use crate::prelude::*;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(fmt::Debug))]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(fmt::Debug))]
struct GlobalFlags {
    // -v, -q flags
    verbosity: Verbosity,
    // -c flag
    colors: bool,
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(fmt::Debug))]
struct ParsedArgs {
    // Safety: if `this` is ever needed, open the path
    // and instantly get it's fd. This is a bit safer
    // because the binary may change while still running.
    // In any case, the usage is discouraged.
    // pub this: Text,
    pub subcommand: Text,
    pub flags: GlobalFlags,
    pub subcommand_flags: Box<[Text]>,
    pub rest_args: Box<[Text]>
}
