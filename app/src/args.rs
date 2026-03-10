//! # Worker: ArgParser
//! ### Here
//! - Command line arguments
//! - Global flags
//! - Sub-command flags

// Note: like clap, accept `--flag x` `--flag=x` `-fx`
// Note: I need some metadata to check if the flag
//       takes a value after it.
// Note: join args with '\0' so its easier to parse.
// Note: as functionality grows I'm considering moving
//       all of this to its own sub-crate.

#[cfg(debug_assertions)]
use std::fmt;
use crate::prelude::*;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(fmt::Debug))]
#[derive(Default)]
pub enum Verbosity {
    Quiet,
    #[default]
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
    // Safety note: if `this` is ever needed, open the path
    // and instantly get it's fd. This is a bit safer
    // because the binary may change while still running.
    // In any case, the usage is discouraged.
    // pub this: Fd,
    pub flags: GlobalFlags,
    pub subcommand: Text,
    pub subcommand_flags: Array<Text>,
    pub rest_args: Array<Text>
}
