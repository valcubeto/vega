use std::num::NonZero;

use prelude::types::*;
use prelude::terminal::*;

// Flag: "--" | "--" ("no-")? (long) (=value)? | '-' (short)+
#[allow(clippy::while_let_on_iterator)]
pub fn drain_flags(args: SliceView<Str>, candidates: SliceView<Flag>) -> (Slice<Str>, Slice<ParsedFlagValue>) {
    let mut args_iter = args.iter().enumerate();
    let mut positionals = vec![];
    let mut flags = vec![];

    while let Some((i, arg)) = args_iter.next() {
        if arg.as_ref() == "--" {
            break;
        }
        // flag starts with "no-"
        if let Some(flag) = arg.strip_prefix("--") {
            let (disabled, flag) = arg.strip_prefix("no-")
                .map_or((false, flag), |flag| (true, flag));
            if let Some(cand) = candidates.iter().find(|cand| flag.starts_with(&cand.long)) {
                let Some(x) = flag.split_once('=') else { panic!() };
            }
            continue;
        }
        if let Some(short_flag) = arg.strip_prefix('-') {
            for letter in short_flag.chars() {
                if !letter.is_ascii_alphanumeric() {
                    panic!("invalid flag at pos {}", i + 1);
                }
            }
            todo!("short flags");
            // continue;
        }
        positionals.push(arg.clone())
    }
    while let Some((_i, arg)) = args_iter.next() {
        positionals.push(arg.clone())
    }
    (positionals.into_boxed_slice(), flags.into_boxed_slice())
}

// pub trait FromArgs {
//     fn from_args(args: &mut RawArgs) -> Self;
// }

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Flag {
    long : String,
    short: Option<NonZero<char>>,
    expected_val: FlagValue
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FlagValue {
    Boolean,
    String
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParsedFlag {
    name: String,
    value: ParsedFlagValue,
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ParsedFlagValue {
    Disabled,
    Enabled,
    String(Str)
}
