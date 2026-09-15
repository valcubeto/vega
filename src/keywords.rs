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
    "fun" => Function,
    "not" => Not,
}
