pub mod options;
pub mod flags;
use options::*;

use crate::flags::FromArgs;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Args {
    options: ProgramOptions,
    subcommand: String,
    args: Vec<String>
}

impl Args {
    pub fn parse() -> Self {
        let mut args = std::env::args()
            .skip(1)              // Ignore self argument.
            .take(64)             // Just in case.
            .collect::<Vec<_>>();
        let options = ProgramOptions::from_args(&mut args);
        let subcommand = String::new();
        Args {
            options,
            subcommand,
            args
        }
    }
}
