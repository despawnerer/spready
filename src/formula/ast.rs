use std::collections::HashSet;

use crate::reference::Reference;

use super::lexer::Span;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Reference(Reference),
    Op(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
}

impl Expr {
    pub fn find_references(&self) -> HashSet<Reference> {
        let mut references = HashSet::new();

        fn _find_references_in_expr(expr: &Expr, target: &mut HashSet<Reference>) {
            match expr {
                Expr::Reference(x) => {
                    target.insert(*x);
                }
                Expr::Op(l, _, r) => {
                    _find_references_in_expr(&l.0, target);
                    _find_references_in_expr(&r.0, target);
                }
                Expr::Integer(_) => {}
                Expr::Float(_) => {}
            }
        }

        _find_references_in_expr(self, &mut references);

        references
    }
}
