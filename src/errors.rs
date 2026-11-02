use crate::common::*;

pub type VegaResult<T> = Result<T, (Position, anyhow::Error)>;

pub trait VegaMoreContext<T> {
    fn at(self, pos: Position) -> VegaResult<T>;
}

impl<T> VegaMoreContext<T> for anyhow::Result<T> {
    fn at(self, pos: Position) -> VegaResult<T> {
        self.map_err(|ctx| (pos, ctx))
    }
}

macro_rules! error_struct {
    ($($name:ident => $repr:literal),* $(,)?) => {
        $(
            pub struct $name<S: ::std::fmt::Display>(pub S);
            impl<S: ::std::fmt::Display> ::std::fmt::Display for $name<S> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    use crossterm::style::Stylize;
                    write!(f, "{}: {}", $repr.red().bold(), self.0)
                }
            }
        )*
    };
}

error_struct! {
    SyntaxError => "Syntax error",
    ParseError  => "Parse error",
    Todo => "Not yet implemented",
}
