use std::collections::HashSet;
use std::str::FromStr;

use grammar::FormulaParser;
use reference::Reference;
use syntax::Expr;
use value::InvalidValue;

#[derive(Debug, Clone)]
pub struct Formula {
    pub expr: Box<Expr>,
}

impl FromStr for Formula {
    type Err = InvalidValue;

    fn from_str(input: &str) -> Result<Formula, InvalidValue> {
        lazy_static! {
            static ref PARSER: FormulaParser = FormulaParser::new();
        }

        let expr = match PARSER.parse(input) {
            Ok(x) => x,
            Err(_) => return Err(InvalidValue),
        };

        Ok(Formula { expr })
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
            }
        };

        _find_references_in_expr(&self.expr, &mut references);

        references
    }
}
