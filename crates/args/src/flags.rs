
pub trait FromArgs {
    fn from_args(args: &mut RawArgs) -> Self;
}

pub type RawArgs = Vec<String>;

pub type Flags = Vec<Flag>;

#[allow(dead_code)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Flag {
    short: Option<char>,
    long: Option<String>
}

