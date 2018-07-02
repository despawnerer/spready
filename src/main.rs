extern crate lalrpop_util;

mod formula;
mod value;
mod reference;

use std::collections::{HashMap, HashSet};

use value::{Value, InvalidValue};
use reference::{Reference, InvalidReference};
use formula::FormulaParser;

// cells

#[derive(Clone, Debug)]
struct Cell {
    text: String,
    value: Result<Value, InvalidValue>,
    successors: HashSet<Reference>,
}

impl Cell {
    fn new(text: String, value: Result<Value, InvalidValue>) -> Cell {
        Cell {
            text, value, successors: HashSet::new()
        }
    }
}

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

    fn enter<T: ToString>(&mut self, reference: Reference, text: T) {
        let text = text.to_string();
        let value = if text.starts_with('=') {
            match self.formula_parser.parse(&text[1..]) {
                Ok(n) => Ok(Value::Integer(n)),
                Err(_) => Err(InvalidValue),
            }
            // println!("{:?}", res);
            // text.parse() // todo
        } else {
            text.parse()
        };

        if let Some(cell) = self.cells.get_mut(&reference) {
            cell.text = text;
            cell.value = value;
            return;
        }

        self.cells.insert(reference, Cell::new(text, value));
    }
}

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.enter("A1".parse().unwrap(), 10);
    spreadsheet.enter("A2".parse().unwrap(), "=20");
    spreadsheet.enter("A3".parse().unwrap(), "=20+30");
    spreadsheet.enter("B1".parse().unwrap(), "=A1+A2+A3");
    println!("{:?}", spreadsheet.cells);
}
