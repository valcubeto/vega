//! # Worker: ArgParser
//! ### Here
//! - Command line arguments
//! - Global flags
//! - Sub-command flags
//!
//! join args with \0 so its easier to parse

use std::fmt;
use crate::prelude::*;

#[derive(fmt::Debug)]
struct GlobalFlags {
    // quiet
    no_output: bool
}

#[allow(dead_code)]
#[derive(fmt::Debug)]
struct ParsedArgs {
    pub this: Text,
    pub subcommand: Text,
    pub flags: GlobalFlags,
    pub subcommand_flags: Box<[Text]>,
    pub rest_args: Box<[Text]>
}
