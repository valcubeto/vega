use crate::common::*;

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    String(Symbol)
}
