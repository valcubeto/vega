pub mod options;
pub mod flags;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParsedArgs {
    pub subcommand: Option<String>,
    pub args: Vec<String>
}

impl ParsedArgs {
    pub fn parse() -> Self {
        let mut args = std::env::args()
            .skip(1)              // Ignore exe path.
            .take(256);           // Just in case.
        let subcommand = args.next();
        ParsedArgs {
            subcommand,
            args: args.collect()
        }
    }
}
