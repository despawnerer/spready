extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate arrayvec;
extern crate regex;

mod evaluate;
mod formula;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod grammar;
mod graph;
mod reference;
mod sheet;
mod syntax;
mod value;

use std::str::FromStr;

use evaluate::evaluate;
use formula::Formula;
use graph::DirectedGraph;
use reference::Reference;
use sheet::{Cell, Sheet};
use value::{MaybeValue, InvalidValue};

#[derive(Debug, Default)]
struct Spreadsheet {
    cells: Sheet,
    dependencies: DirectedGraph,
}

impl Spreadsheet {
    pub fn new() -> Spreadsheet {
        Default::default()
    }

    pub fn get<R: Into<Reference>>(&self, reference: R) -> Option<&Cell> {
        self.cells.get(&reference.into())
    }

    pub fn get_guaranteed<R: Into<Reference>>(&mut self, reference: R) -> &mut Cell {
        self.cells
            .entry(reference.into())
            .or_insert_with(Cell::default)
    }

    pub fn enter<R, T>(&mut self, reference: R, input: T)
    where
        R: Into<Reference>,
        T: ToString,
    {
        let reference = reference.into();
        let input = input.to_string();

        if input.starts_with('=') {
            match Formula::from_str(&input) {
                Ok(formula) => {
                    self.dependencies.set_incoming_edges(reference, formula.find_references());
                    let cell = self.get_guaranteed(reference);
                    cell.input = input;
                    cell.formula = Some(formula);
                },
                Err(_) => {
                    let cell = self.get_guaranteed(reference);
                    cell.value = Err(InvalidValue);
                    cell.input = input;
                    cell.formula = None;
                }
            }
        } else {
            let cell = self.get_guaranteed(reference);
            cell.value = input.parse();
            cell.input = input;
            cell.formula = None;
        }

        self.recalculate();
    }

    fn recalculate(&mut self) {
        let order = self.dependencies.to_topological_sort().unwrap();
        for reference in order.iter().cloned() {
            if let Some(value) = self.calculate_cell(reference) {
                self.get_guaranteed(reference).value = value
            }
        }
    }

    fn calculate_cell(&self, reference: Reference) -> Option<MaybeValue> {
        self.get(reference).and_then(|cell| cell.formula.as_ref()).map(|formula| evaluate(&formula.expr, &self.cells))
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
    spreadsheet.enter("C3", "=A4 + 20");

    spreadsheet.enter("F1", "=AB6001");

    for (reference, cell) in &spreadsheet.cells {
        println!("{:?} :: {:?}", reference, cell);
    }

    println!(
        "Order of evaluation: {:?}",
        spreadsheet.dependencies.to_topological_sort()
    );
}
