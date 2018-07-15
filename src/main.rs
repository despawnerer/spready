extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;

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
use value::InvalidValue;

#[derive(Debug, Default)]
struct Spreadsheet {
    cells: Sheet,
    dependencies: DirectedGraph<Reference>,
}

impl Spreadsheet {
    pub fn new() -> Spreadsheet {
        Default::default()
    }

    pub fn get<R: Into<Reference>>(&self, reference: R) -> Option<&Cell> {
        self.cells.get(&reference.into())
    }

    pub fn get_mut<R: Into<Reference>>(&mut self, reference: R) -> Option<&mut Cell> {
        self.cells.get_mut(&reference.into())
    }

    pub fn get_or_empty_mut<R: Into<Reference>>(&mut self, reference: R) -> &mut Cell {
        self.cells
            .entry(reference.into())
            .or_insert_with(Cell::default)
    }

    pub fn set<R, T>(&mut self, reference: R, input: T)
    where
        R: Into<Reference>,
        T: ToString,
    {
        let reference = reference.into();
        let input = input.to_string();

        let (value, formula) = if input.starts_with('=') {
            match Formula::from_str(&input) {
                Ok(formula) => (evaluate(&formula.expr, &self.cells), Some(formula)),
                Err(_) => (Err(InvalidValue), None),
            }
        } else {
            (input.parse(), None)
        };

        if let Some(ref formula) = formula {
            self.dependencies
                .set_incoming_edges(reference, formula.find_references());
        }

        let cell = self.get_or_empty_mut(reference);
        cell.input = input;
        cell.value = value;
        cell.formula = formula;
    }
}

fn main() {
    let mut spreadsheet = Spreadsheet::new();
    spreadsheet.set("A1", 10);
    spreadsheet.set("A2", 20);
    spreadsheet.set("A3", 30);

    spreadsheet.set("B1", "=20");
    spreadsheet.set("B2", "=30+50");

    spreadsheet.set("C1", "=A1*2");
    spreadsheet.set("C2", "=A1+A2+A3");
    spreadsheet.set("C3", "=A4 + 20");

    for (reference, cell) in &spreadsheet.cells {
        println!("{:?} :: {:?}", reference, cell);
    }

    println!("Dependencies: {:?}", spreadsheet.dependencies);
    println!(
        "Sorted topologically: {:?}",
        spreadsheet.dependencies.to_topological_sort()
    );
}
