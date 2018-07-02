extern crate lalrpop_util;

mod formula;
mod value;
mod reference;
mod ast;
mod cell;

use std::collections::{HashMap, HashSet};

use value::{Value, InvalidValue};
use reference::{Reference, InvalidReference};
use cell::Cell;
use formula::FormulaParser;
use ast::Expr;

// spreadsheet

struct Spreadsheet {
    cells: HashMap<Reference, Cell>,
    formula_parser: FormulaParser,
}

impl Spreadsheet {
    fn new() -> Spreadsheet {
        Spreadsheet {
            cells: HashMap::default(),
            formula_parser: FormulaParser::new(),
        }
    }

    fn enter<R, T>(&mut self, reference: R, input: T)
    where R: Into<Reference>,
        T: ToString {
        let reference = reference.into();
        let input = input.to_string();

        let value = if input.starts_with('=') {
            match self.formula_parser.parse(&input) {
                Ok(expr) => expr.evaluate(&self.cells),
                Err(_) => Err(InvalidValue),
            }
        } else {
            input.parse()
        };

        if let Some(cell) = self.cells.get_mut(&reference) {
            cell.input = input;
            cell.value = value;
            return;
        }

        self.cells.insert(reference, Cell::new(input, value));
    }
}

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.enter("A1", 10);
    spreadsheet.enter("A2", 20);
    spreadsheet.enter("A3", 30);

    spreadsheet.enter("B1", "=20");
    spreadsheet.enter("B2", "=30+50");

    spreadsheet.enter("C1", "=A1*2");
    spreadsheet.enter("C2", "=A1+A2+A3");

    spreadsheet.enter("D1", "=sum(A1:A3)");

    println!("{:?}", spreadsheet.cells);
}
