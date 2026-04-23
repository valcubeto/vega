macro_rules! keywords {
    ($( $kw_name:ident = $kw_string:literal; )+) => {
        pub enum Keyword {
            $( $kw_name, )+
        }
        impl Keyword {
            #[allow(clippy::should_implement_trait)]
            pub fn from_str(string: &str) -> Option<Self> {
                match string {
                    $( $kw_string => Some(Self::$kw_name), )+
                    _ => None
                }
            }
            pub fn is_keyword(string: &str) -> bool {
                matches!(string, $( $kw_string )|+)
            }
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( Self::$kw_name => $kw_string ),+
                }
            }
        }
    };
}

keywords! {
    InlineMacro = "def";
    Macro = "macro";
    TypeDef = "type";

    It = "it";

    Alias       = "as";
    TypeMatches = "is";

    Module = "mod";
    Import = "use";
    // "include" is a macro

    Public = "public";

    Generator = "gen";

    Async = "async";
    // await is special, it's more like a suffix operator
    // Await = "await";

    If   = "if";
    Else = "else";

    Match = "match";

    Loop  = "loop";
    While = "while";
    For   = "for";
    ForIn = "in";

    With = "with";

    Catch = "catch";

    LogicAnd = "and";
    LogicOr  = "or";
    LogicNeg = "not";

    ConstGlobal = "const";
    MutGlobal   = "state";

    ConstLocal  = "let";
    MutLocal    = "var";

    Struct    = "struct";
    Enum      = "enum";
    Interface = "interface";

    Function = "fun";

    Where = "where";
}
