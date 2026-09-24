macro_rules! keywords {
    ($($str:literal => $ident:ident),+ $(,)?) => {
        #[cfg_attr(debug_assertions, derive(Debug))]
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Keyword {
            $( #[doc = concat!("Keyword \\``", $str, "`\\`")] $ident, )+
        }

        #[allow(dead_code)]
        impl Keyword {
            pub fn from_str(word: &str) -> Option<Self> {
                match word {
                    $( $str => Some(Keyword::$ident), )+
                    _ => None
                }
            }

            pub fn as_str(self) -> &'static str {
                match self {
                    $( Self::$ident => $str, )+
                }
            }

            pub fn len(self) -> usize {
                self.as_str().len()
            }
        }
    };
}

keywords! {
    // Imports
    "use" => Import,
    "as" => Alias,

    // Modifiers
    // "const" for functions
    "async" => Async,
    "await" => Await,

    // Declarations
    "const" => StaticConst,
    "state" => StaticVar,
    "let" => LocalConst,
    "var" => LocalVar,
    "fun" => Function,
    "struct" => Struct,
    "enum" => Enum,
    "union" => Union,
    "interface" => Interface,
    "type" => TypeAlias,
    "impl" => Implement,

    // Control
    "do" => DoBlock,
    "assert" => Assert,
    "match" => Match,
    "if" => If,
    "else" => Else,
    "return" => Return, // To return Ok/Some
    "yield" => Yield,
    "throw" => Throw,   // To return Err/None
    "break" => Break,
    "continue" => Continue,
    "while" => While,
    "for" => For,
    "in" => In,
    "try" => TryBlock,

    // Ops
    "not" => Not,
    "and" => And,
    "or" => Or,

    // Other
}
