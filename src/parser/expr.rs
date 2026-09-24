use super::Op;
#[cfg(debug_assertions)]
use {
    std::fmt,
    crossterm::style::Stylize
};

use num_bigint::BigInt;
use bigdecimal::BigDecimalRef;

pub struct Expr<'s, 'h> {
    pub kind: ExprKind<'s, 'h>,
}

#[allow(dead_code)]
pub enum ExprKind<'s, 'h> {
    Ident(&'s str),
    String(&'s str),
    Integer(&'h BigInt),
    Decimal(BigDecimalRef<'h>),
    UnaryOp {
        op: Op,
        value: Box<Expr<'s, 'h>>,
    },
    BinaryOp {
        op: Op,
        lhs: Box<Expr<'s, 'h>>,
        rhs: Box<Expr<'s, 'h>>,
    },
    Call {
        lhs: Box<Expr<'s, 'h>>,
        args: Box<[Expr<'s, 'h>]>,
    },
    Index {
        lhs: Box<Expr<'s, 'h>>,
        idx: Box<Expr<'s, 'h>>,
    },
    Construct {
        lhs: Box<Expr<'s, 'h>>,
        fields: Box<[Expr<'s, 'h>]>
    },
    If {
        condition: Box<Expr<'s, 'h>>,
        body: Box<[Expr<'s, 'h>]>,
        else_block: Box<[Expr<'s, 'h>]>,
    },
}

#[cfg(debug_assertions)]
impl<'s, 'h> fmt::Debug for Expr<'s, 'h> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Ident(ident) => write!(f, "{}", ident.underlined()),
            ExprKind::String(string) => write!(f, "{}", format!("{:?}", string).yellow()),
            ExprKind::Integer(n) => write!(f, "{} {{{}}}", "Integer".cyan(), n.to_string().blue()),
            ExprKind::Decimal(n) => write!(f, "{} {{{}}}", "Decimal".cyan(), format!("{n:.8}").blue()),
            ExprKind::UnaryOp { op, value } => write!(f, "({:?} {:?})", op, value),
            ExprKind::BinaryOp { op, lhs, rhs } => write!(f, "({:?} {:?} {:?})", lhs, op, rhs),
            ExprKind::Call { lhs, args } => write!(f, "(CALL {:?} ({:?}))", lhs, args),
            ExprKind::Index { lhs, idx } => write!(f, "(IDX {:?}[{:?}])", lhs, idx),
            ExprKind::Construct { lhs, fields } => write!(f, "(CONSTRUCT {:?} {{ {:?} }})", lhs, fields),
            ExprKind::If { condition, body, else_block } => write!(f, "{} {:?} {:#?} else {:#?};", "if".magenta(), condition, body, else_block),
        }
    }
}
