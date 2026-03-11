#[doc(hidden)]
pub fn debug_header(file: &str, line: u32, column: u32) {
    use deps::owo_colors::OwoColorize;
    // Note: should I add timestamps?
    println!("[{}: {}:{}:{}]", "debug".bright_yellow().bold(), file.bold(), line, column);
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
            use $crate::__macro_deps::OwoColorize;
            $crate::debugging::debug_header(file!(), line!(), column!());
            #[allow(clippy::useless_format)]
            println!("    {} {}", "#".blue().italic(), format!($msg, $($($value),+)?).blue().italic());
        }
    };
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            use $crate::__macro_deps::OwoColorize;
            use $crate::debugging::Indent;
            $crate::debugging::debug_header(file!(), line!(), column!());
            println!("{}", format!("{} {} {}", stringify!($val).bold(), "=".blue(), format!("{:#?}", $val)).indent(4));
        }
    };
}

pub trait Indent<T: AsRef<str>> {
    fn indent(&self, n: usize) -> String;
}

impl<T> Indent<T> for T where T: AsRef<str> {
    fn indent(&self, n: usize) -> String {
        self.as_ref().lines()
            .map(|line| " ".repeat(n) + line)
            .collect::<Vec<String>>()
            .join("\n")
    }
}
