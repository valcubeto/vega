use std::fmt;
use crossterm::style::Stylize;
use num_bigint::BigInt;
use bigdecimal::{ BigDecimal, BigDecimalRef };

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Value<'rt> {
    Unit,
    Ident(&'rt str),
    String(&'rt str),
    Integer(&'rt BigInt),
    Decimal(BigDecimalRef<'rt>),
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Ident(ident) => write!(f, "{ident}"),
            Value::String(string) => write!(f, "{}", format!("{string:?}").yellow()),
            Value::Integer(n) => write!(f, "{}", n.to_string().blue()),
            Value::Decimal(n) => write!(f, "{}", format!("{n:.2}").blue()),
        }
    }
}
