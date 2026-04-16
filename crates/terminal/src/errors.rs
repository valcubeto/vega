macro_rules! define_errors {
    ($_:tt $( $name:ident!($repr:literal); )+) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($_ msg:literal $_(, $_($_ value:expr),+)?) => {{
                    use $_ crate::_macro_deps::OwoColorize;
                    #[allow(clippy::useless_format)]
                    eprintln!("{}: {}", $repr.bright_red().bold(), format!($_ msg $_(, $_($_ value),+)?));
                    ::std::process::exit(1);
                }};
            }
        )+
    };
}

define_errors! { $
    fatal!("Error");
    syntax_err!("Syntax error");
    todo_err!("Not yet implemented");
}
