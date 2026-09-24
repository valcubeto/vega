mod values; pub use values::*;
mod runtime; pub use runtime::*;
use crate::{ parser::{ Expr, ExprKind, Op }, error::*, };
use std::{ path::Path, slice::Iter };

pub struct Interpreter<'a, 's, 'n, 'h> {
    pub file: &'a Path,
    rt: &'h mut Runtime<'s, 'h>,
    ast: Iter<'n, Expr<'s>>,
}

impl<'a, 's, 'n, 'h>
    Interpreter<'a, 's, 'n, 'h>
where
    'n: 's
    // 'h: 's,
    // 's: 'a,
    // 'n: 'a,
{
    pub fn new(file: &'a Path, rt: &'h mut Runtime<'s, 'h>, ast: &'n [Expr<'s>]) -> Self {
        Interpreter {
            file,
            rt,
            ast: ast.iter()
        }
    }

    pub fn eval_scope(mut self) -> Result<'a, Value<'s, 'h>> {
        #[allow(clippy::never_loop)]
        while let Some(node) = self.ast.next() {
            let result = self.eval(node)?;
            // This loop never loops blah blah
            return Ok(result);
            // println!("(interpreter::Interpreter::eval_scope) result = {result:?}");
        }
        Ok(Value::Unit)
    }

    fn get_value(&self, key: &str) -> Result<'a, Value<'s, 'h>> {
        self.rt.env.get(&key).ok_or(self.error("Not found", Marker::Span(0, 0))).copied()
    }

    // fn get_value_mut(&mut self) {}

    pub fn eval(&mut self, expr: &'n Expr<'s>) -> Result<'a, Value<'s, 'h>> {
        let result = match expr.kind {
            ExprKind::Ident(ident) => self.get_value(ident)?,
            ExprKind::String(string) => Value::String(self.rt.heap.alloc_str(string)),
            // TODO: no clone needed
            ExprKind::Integer(n) => {
                Value::Integer(self.rt.heap.alloc(n))
            }
            // ExprKind::Ident(ident) => Value::Ident(ident),
            ExprKind::BinaryOp { op, lhs, rhs } => {
                match op {
                    Op::Assign => {
                        // let lhs = self.rt.env.entry()
                        todo!()
                    }
                    _ => todo!()
                }
            }
            _ => {
                let e: Error<'a> = self.todo(format!("expr {expr:?}"), Marker::Span(0, 0));
                return Err(e)
            },
        };
        Ok(result)
    }

    fn error(&self, message: impl std::fmt::Display, pos: Marker) -> Error<'a> {
        Error {
            kind: ErrorKind::EvalError,
            message: message.to_string().into_boxed_str(),
            file: self.file,
            pos,
        }
    }

    fn todo(&self, message: impl std::fmt::Display, pos: Marker) -> Error<'a> {
        Error {
            kind: ErrorKind::NotYetImplemented,
            message: message.to_string().into_boxed_str(),
            file: self.file,
            pos,
        }
    }
}
