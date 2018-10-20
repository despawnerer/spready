use std::collections::HashSet;
use std::str::FromStr;

use grammar::FormulaParser;
use reference::Reference;
use syntax::Expr;
use value::EvaluationError;

#[derive(Debug, Clone)]
pub struct Formula {
    pub expr: Box<Expr>,
}

impl FromStr for Formula {
    type Err = EvaluationError;

    fn from_str(input: &str) -> Result<Formula, EvaluationError> {
        lazy_static! {
            static ref PARSER: FormulaParser = FormulaParser::new();
        }

        PARSER
            .parse(input)
            .map(|expr| Formula { expr })
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
                    _find_references_in_expr(l, target);
                    _find_references_in_expr(r, target);
                }
                Expr::Integer(_) => {}
                Expr::Float(_) => {}
            }
        };

        _find_references_in_expr(&self.expr, &mut references);

        references
    }
}
