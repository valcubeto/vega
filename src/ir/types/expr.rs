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
        target: Box<Expr>,
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
}
