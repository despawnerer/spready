use std::collections::HashSet;
use std::str::FromStr;

use chumsky::stream::Stream;
use chumsky::Parser;

use crate::lexer::lexer;
use crate::parser::{expr_parser, Expr};
use crate::reference::Reference;
use crate::value::EvaluationError;

#[derive(Debug, Clone)]
pub struct Formula {
    pub expr: Box<Expr>,
}

impl FromStr for Formula {
    type Err = EvaluationError;

    fn from_str(input: &str) -> Result<Formula, EvaluationError> {
        // lazy_static! {
        //     static ref PARSER: FormulaParser = FormulaParser::new();
        // }

        let len = input.chars().count();

        let tokens = lexer().parse(input).map_err(|e| {
            println!("{:?}", e);

            EvaluationError::ParseError
        })?;

        println!("{:?}", tokens);

        expr_parser()
            .parse(Stream::from_iter(len..len + 1, tokens.into_iter()))
            .map(|v| {
                println!("{:?}", v);
                v
            })
            .map(|expr| Formula {
                expr: Box::new(expr.0),
            })
            .map_err(|_| EvaluationError::ParseError)
    }
}

impl Formula {
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
                Expr::Value(_) => {}
            }
        }

        _find_references_in_expr(&self.expr, &mut references);

        references
    }
}
