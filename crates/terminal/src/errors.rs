#[macro_export]
macro_rules! fatal {
    ($msg:literal $(, $($value:expr),+)?) => {
        use $crate::_macro_deps::OwoColorize;
        #[allow(clippy::useless_format)]
        eprintln!("{}: {}", "Error".bright_red().bold(), format!($msg $(, $($value),+)?));
        ::std::process::exit(1);
    };
    (@ $name:literal, $msg:literal $(, $($value:expr),+)?) => {
        use $crate::_macro_deps::OwoColorize;
        #[allow(clippy::useless_format)]
        eprintln!("{}: {}", $name.bright_red().bold(), format!($msg $(, $($value),+)?));
        ::std::process::exit(1);
    };
}

macro_rules! other_errors {
    ($doll:tt, $($name:ident!($repr:literal)),+) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($msg:literal $doll(, $doll($value:expr),+)?) => {
                    $crate::fatal!(@ $repr, $msg $doll(, $doll($value),+)?)
                };
            }
        )+
    };
}

other_errors! { $,
    syntax_err!("Syntax error")
}
