use chumsky::prelude::*;
use chumsky::stream::Stream;

use crate::value::EvaluationError;

use super::ast::{BinaryOp, Expr, Spanned};
use super::lexer::{lexer, Token};

pub fn parse_formula(input: &str) -> Result<Expr, EvaluationError> {
    let len = input.chars().count();

    let tokens = lexer()
        .parse(input)
        .map_err(|e| EvaluationError::ParseError)?;

    expr_parser()
        .parse(Stream::from_iter(len..len + 1, tokens.into_iter()))
        .map_err(|_| EvaluationError::ParseError)
        .map(|expr| expr.0) // discard the span
}

fn expr_parser() -> impl Parser<Token, Spanned<Expr>, Error = Simple<Token>> + Clone {
    recursive(
        |expr: Recursive<Token, (Expr, std::ops::Range<usize>), Simple<Token>>| {
            let val = select! {
                Token::Integer(n) => Expr::Integer(n.parse().unwrap()),
                Token::Float(s) => Expr::Float(s.parse().unwrap()),
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
