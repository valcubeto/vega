macro_rules! operators {
    (
        infix   = { $( $infix_name  :ident = $infix_string  :literal; )+ }
        prefix  = { $( $prefix_name :ident = $prefix_string :literal; )+ }
        postfix = { $( $postfix_name:ident = $postfix_string:literal; )+ }
    ) => {
        pub enum Operator {
            $( $infix_name,   )+
            $( $prefix_name,  )+
            $( $postfix_name  ),+
        }
        impl Operator {
            pub fn parse() {
                todo!();
            }
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( Self::$infix_name   => concat!(" ", $infix_string,   " "), )+
                    $( Self::$prefix_name  => concat!(" ", $prefix_string      ), )+
                    $( Self::$postfix_name =>              $postfix_string        ),+
                }
            }
        }
    };
}

operators! {
    infix = {
        Add = "+";
        Sub = "-";
        Mul = "*";
        Pow = "**";
        Mod = "%";
        Div = "/";

        BitAnd = "&";
        BitOr  = "|";

        Equals    = "=";
        AddEquals = "+=";
        SubEquals = "-=";

        UpTo  = "..";
        Until = "..<";

        Clamp = "<>";

        LessThan    = "<";
        GreaterThan = ">";
        LessOrEqualTo    = "<=";
        GreaterOrEqualTo = ">=";

        TryOrElse = "?:";

        ItemAccess = "::";
        PropAccess = ".";
    }
    prefix = {
        BitNeg = "~";
    }
    postfix = {
        Try = "?";
    }
}
