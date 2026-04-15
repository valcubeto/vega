#[macro_export]
/// Shorten static string constant definitions.
macro_rules! define {
    ($($name:ident = $value:expr;)+) => {
        $(
            pub const $name: &str = $value;
        )*
    };
}

define! {
    NAME = "vega";
    NAME_CAPITALIZED = "Vega";
    VERSION = env!("CARGO_PKG_VERSION");
}
