#[doc(hidden)]
pub fn debug_header(file: &str, line: u32, column: u32) {
    use owo_colors::OwoColorize;
    println!("[{}: {}:{}:{}]", "debug".bright_yellow().bold(), file, line, column);
}

/// `debug!(<literal>, <values...>)`:
/// - Prints a comment-like message.
/// - Behaves like `println!`
///
/// `debug!(<expr>)`:
/// - Prints `<expr> = <result>`.
/// - Behaves like `dbg!`
///
/// Both with a nice header.
///
/// Example:
/// ```rs
/// // [debug: path:x:y]
/// //     # This is a message
/// debug!("This is a message");
///
/// // [debug: path:x:y]
/// //     # Running crate v1.0.0
/// debug!("Running crate v{}", env!("CARGO_PKG_VERSION"));
///
/// // [debug: path:x:y]
/// //     1 + 1 = 2
/// debug!(1 + 1);
/// ```
#[macro_export]
macro_rules! debug {
    ($msg:literal $(, $($value:expr),+)?) => {
        #[cfg(debug_assertions)]
        {
            use $crate::_macro_deps::OwoColorize;
            $crate::debugging::debug_header(file!(), line!(), column!());
            // I don't think clippy will ever be able to
            // catch this, but just in case.
            #[allow(clippy::useless_format)]
            println!("    {} {}", "#".blue().italic(), format!($msg, $($($value),+)?).blue().italic());
        }
    };
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            use $crate::_macro_deps::OwoColorize;
            use $crate::debugging::{ indent, debug_header };
            debug_header(file!(), line!(), column!());
            let msg = format!("{} {} {}", stringify!($val).bold(), "=".blue(), format!("{:#?}", $val));
            println!("{}", indent(msg, 4));
        }
    };
}

/*
 * NOTE:
 * I just wanted to use it quickly.
 * Sorry if it looks JavaScript-ish.
 */
pub fn indent(string: impl AsRef<str>, n: usize) -> String {
    string.as_ref().split('\n')
          .map(|line| " ".repeat(n) + line)
          .collect::<Vec<String>>()
          .join("\n")
}
