macro_rules! keywords {
    ($($str:literal => $ident:ident);+) => {
        #[cfg_attr(debug_assertions, derive(Debug))]
        #[derive(Clone, Copy)]
        pub enum Keyword {
            $( #[doc = concat!("`", $str, "`")] $ident, )+
        }
        impl Keyword {
            pub fn from_str(word: &str) -> Option<Self> {
                match word {
                    $( $str => Some(Keyword::$ident), )+
                    _ => None
                }
            }
            pub fn as_str(self) -> &'static str {
                match self {
                    $( Self::$ident => $str )+
                }
            }
            pub fn len(self) -> usize {
                self.as_str().len()
            }
        }
    };
}

keywords! {
    "fun" => Function
}
