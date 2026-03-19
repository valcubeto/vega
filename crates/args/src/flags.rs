
#[allow(clippy::while_let_on_iterator)]
pub fn drain_flags(args: &RawArgs, candidates: Flags) -> (Vec<String>, Vec<FlagValue>) {
    let mut args_iter = args.iter().enumerate();
    let mut positionals = vec![];
    let mut flags = vec![];

    // Avoid that implicit call to into_iter.
    while let Some((i, arg)) = args_iter.next() {
        if arg == "--" {
            break;
        }
        if let Some(long_flag) = arg.strip_prefix("--") {
            if let Some(flag) = candidates.iter().find(|flag| long_flag.starts_with(&flag.long)) {
                flags.push(match flag.takes {
                    PossibleFlagValue::Disables => FlagValue::False,
                    PossibleFlagValue::Enables  => FlagValue::True,
                    PossibleFlagValue::String   => {
                        match long_flag.as_bytes().get(flag.long.len()) {
                            Some(b'=') => FlagValue::String(
                                long_flag[..flag.long.len() + 1].to_string()
                            ),
                            None => FlagValue::String(
                                args_iter.next().expect("flag requires a value").1.clone()
                            ),
                            Some(_) => panic!("unknown flag")
                        }
                    }
                });
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
    (positionals, flags)
}

pub trait FromArgs {
    fn from_args(args: &mut RawArgs) -> Self;
}

pub type RawArgs = [String];

pub type Flags<'a> = &'a [PossibleFlag];

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PossibleFlag {
    long: String,
    short: Option<char>,
    takes: PossibleFlagValue
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum PossibleFlagValue {
    Disables,
    Enables,
    String
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Flag {
    name: String,
    value: FlagValue
}

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum FlagValue {
    False,
    True,
    String(String)
}
