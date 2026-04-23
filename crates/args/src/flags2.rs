use prelude::types::*;
use std::num::NonZero;
use hashbrown::HashMap;

pub type ParsedFlags = HashMap<Str, ParsedFlag>;

pub struct FlagLayout {
    pub long: String,
    pub short: NonZero<char>,
    pub expected: FlagValue
}

pub enum FlagValue {
    Text,
    Boolean,
}

impl FlagLayout {
    pub fn can_be_negated(&self) -> bool {
        matches!(self.expected, FlagValue::Boolean)
    }
}

pub enum ParsedFlag {
    Text(Str),
    Boolean(bool)
}

