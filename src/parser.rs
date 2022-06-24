use chumsky::prelude::*;

use crate::lexer::{Span, Token};
use crate::reference::Reference;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Value(Value),
    Reference(Reference),
    Op(Box<Spanned<Self>>, BinaryOp, Box<Spanned<Self>>),
}

pub fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    recursive(
        |expr: Recursive<Token, (Expr, std::ops::Range<usize>), Simple<Token>>| {
            let val = select! {
                Token::Integer(n) => Expr::Value(Value::Integer(n.parse().unwrap())),
                Token::Float(s) => Expr::Value(Value::Float(s.parse().unwrap())),
            }
            .labelled("value");

            let reference = select! { Token::Reference(reference) => Expr::Reference(reference.parse().unwrap()) }.labelled("reference");

            // 'Atoms' are expressions that contain no ambiguity
            let atom = val
                .or(reference)
                .map_with_span(|expr, span| (expr, span))
                // Atoms can also just be normal expressions, but surrounded with parentheses
                .or(expr
                    .clone()
                    .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))));

            // Product ops (multiply and divide) have equal precedence
            let op = just(Token::Op('*'))
                .to(BinaryOp::Mul)
                .or(just(Token::Op('/')).to(BinaryOp::Div));

            let product = atom
                .clone()
                .then(op.then(atom).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Op(Box::new(a), op, Box::new(b)), span)
                });

            // Sum ops (add and subtract) have equal precedence
            let op = just(Token::Op('+'))
                .to(BinaryOp::Add)
                .or(just(Token::Op('-')).to(BinaryOp::Sub));

            let sum = product
                .clone()
                .then(op.then(product).repeated())
                .foldl(|a, (op, b)| {
                    let span = a.1.start..b.1.end;
                    (Expr::Op(Box::new(a), op, Box::new(b)), span)
                });

            sum
        },
    )
}
