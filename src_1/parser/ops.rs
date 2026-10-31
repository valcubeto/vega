use crate::keywords::Keyword;
use super::TokenKind;

use std::fmt;
use crossterm::style::Stylize;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Assign,

    // x and y
    And,
    // x or y
    Or,

    // not x
    Not,

    Eq,
    NotEq,
    Less,
    LessEq,
    Greater,
    GreaterEq,

    BitAnd,
    BitOr,
    BitXor,

    // x + y
    Add,
    // x - y
    Sub,

    // x * y
    Mul,
    //  x / y
    Div,

    // x % y
    Mod,

    // x.pow(y)
    // Pow,

    // -x
    Neg,
    // &x
    Ref,
    // *x
    Deref,
    // ~x
    BitNot,

    // x?
    TryVal,

    // x(y)
    Call,
    // x[y]
    Index,
    // x { y }
    Construct,

    // x::y, x.y
    GetItem, GetProp,
}

impl Op {
    pub fn prefix<'s>(token: &TokenKind<'s>) -> Option<Op> {
        use TokenKind as K;
        use Keyword as Kw;
        match token {
            K::Keyword(Kw::Not) => Some(Op::Not),
            _ => None
        }
    }

    pub fn infix<'s>(token: &TokenKind<'s>) -> Option<Op> {
        use TokenKind as K;
        // use Keyword as Kw;
        match token {
            K::Plus => Some(Op::Add),
            K::Minus => Some(Op::Sub),
            K::Star => Some(Op::Mul),
            K::Slash => Some(Op::Div),
            _ => None
        }
    }

    pub fn is_suffix(self) -> bool {
        matches!(self, Op::Not | Op::Ref | Op::Deref | Op::Neg | Op::BitNot)
    }

    /* not x::y.z() + 5 * 3 % 9 and w > d[5]? */
    pub fn bp(self) -> BindingPower {
        // Greater rbp means expressions are grouped LTR ((x + y) + z)
        // Greater lbp means expressions are grouper RTL (x = (y = z))
        // 0 = no binding power
        // lbp < rbp = break
        let (lbp, rbp) = match self {
            Op::Assign => (1, 2),

            Op::And | Op::Or => (20, 21),
            Op::Not => (0, 35),
            Op::Eq | Op::NotEq | Op::Less | Op::LessEq | Op::Greater | Op::GreaterEq => (40, 41),

            Op::BitAnd | Op::BitOr | Op::BitXor => (45, 46),
            Op::Add | Op::Sub => (50, 51),
            Op::Mul | Op::Div => (60, 61),
            Op::Mod => (65, 66),
            // Op::Pow => (70, 71),
            Op::Neg | Op::Ref | Op::Deref | Op::BitNot => (0, 80),

            Op::TryVal => (90, 91),
            // T { v }[i](v)
            Op::Call | Op::Index | Op::Construct => (100, 101),
            // m::I.p
            Op::GetProp | Op::GetItem => (110, 111),
        };

        BindingPower { lbp, rbp }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Op::Assign => "assign",
            Op::And => "and",
            Op::Or => "or",
            Op::Not => "not",
            Op::Eq => "equals",
            Op::NotEq => "not equals",
            Op::Less => "less than",
            Op::LessEq => "less than or equal to",
            Op::Greater => "greater than",
            Op::GreaterEq => "greater than or equal to",
            Op::BitOr => "bitwise or",
            Op::BitXor => "bitwise xor",
            Op::Add => "add",
            Op::Sub => "substract",
            Op::Neg => "negate",
            Op::Mul => "multiplication",
            Op::Ref => "ref",
            Op::Deref => "deref",
            Op::Div => "divide",
            Op::Mod => "modulo",
            Op::BitAnd => "bitwise and",
            Op::BitNot => "bitwise not",
            Op::TryVal => "try",
            // Op::Pow => "power",
            Op::Call => "(call)",
            Op::Index => "(index)",
            Op::Construct => "(construct)",
            Op::GetItem => "get item",
            Op::GetProp => "get property",
        };
        write!(f, "{repr:?}")
    }
}

#[cfg(debug_assertions)]
impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Op::Assign => "=",
            Op::And => "and",
            Op::Or => "or",
            Op::Not => "not",
            Op::Eq => "==",
            Op::NotEq => "!=",
            Op::Less => "<",
            Op::LessEq => "<=",
            Op::Greater => ">",
            Op::GreaterEq => ">=",
            Op::BitOr => "|",
            Op::BitXor => "^",
            Op::Add => "+",
            Op::Sub | Op::Neg => "-",
            Op::Mul | Op::Deref => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::BitAnd | Op::Ref => "&",
            Op::BitNot => "~",
            Op::TryVal => "?",
            // Op::Pow => "**",
            Op::Call => "(call)",
            Op::Index => "(index)",
            Op::Construct => "(construct)",
            Op::GetItem => "::",
            Op::GetProp => ".",
        };
        write!(f, "{}", repr.magenta())
    }
}

/// Binding PAWA!!
pub struct BindingPower {
    pub lbp: u8,
    pub rbp: u8,
}
