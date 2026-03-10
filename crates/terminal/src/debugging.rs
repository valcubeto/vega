

#[doc(hidden)]
pub fn debug_header(file: &str, line: u32, column: u32) {
    use deps::owo_colors::OwoColorize;
    // TODO: add timestamp
    println!("[{}: {}:{}:{}]", "debug".bright_yellow().bold(), file.bold(), line, column);
}

#[macro_export]
macro_rules! debug {
    ($msg:literal) => {{
        #[cfg(debug_assertions)]
        {
            use $crate::__macro_deps::OwoColorize;
            $crate::debugging::debug_header(file!(), line!(), column!());
            println!("    {} {}", "#".blue().italic(), $msg.blue().italic());
        }
    }};
    ($val:expr) => {{
        #[cfg(debug_assertions)]
        {
            use $crate::OwoColorize;
            $crate::debugging::debug_header(file!(), line!(), column!());
            println!("    {} {} {:?}", stringify!($val).bold(), "=".blue(), $val);
        }
    }}
}
