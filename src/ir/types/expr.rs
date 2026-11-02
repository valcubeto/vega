use crate::common::*;
use super::ops::Op;

#[derive(Debug)]
pub struct Expr {
    pub pos: Position,
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Char(char),
    Ident(Symbol),
    String(Symbol),
    UnaryOp {
        op: Op,
        value: Box<Expr>,
    },
    BinaryOp {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        body: Box<[Expr]>,
        else_block: Box<[Expr]>,
    },
}

impl ExprKind {
    #[inline(always)]
    pub fn at(self, pos: Position) -> Expr {
        Expr {
            pos,
            kind: self,
        }
    }

    pub fn debug_with(&self, text: &Interner) -> String {
        match self {
            Self::Char(ch) => format!("{:?}", ch).yellow().to_string(),
            Self::String(sym) => format!("{:?}", text.lookup(*sym).unwrap_or("<not found>")).yellow().to_string(),
            Self::Ident(sym) => format!("{}", text.lookup(*sym).unwrap_or("<not found>").underlined()),
            Self::BinaryOp { op, lhs, rhs } => format!("({} {:?} {})", lhs.kind.debug_with(text), op, rhs.kind.debug_with(text)),
            Self::UnaryOp { op, value } => format!("({:?} {})", op, value.kind.debug_with(text)),
            // Self::If { condition, body, else_block } => format!()
            _ => String::from("<unimplemented>"),
        }
    }
}
