use std::str::FromStr;

use cell::{Cell, Sheet};
use evaluate::evaluate;
use formula::Formula;
use graph::DirectedGraph;
use reference::Reference;
use value::{InvalidValue, MaybeValue};

#[derive(Debug, Default)]
pub struct Spreadsheet {
    pub cells: Sheet,
    pub dependencies: DirectedGraph,
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
                    self.dependencies
                        .set_incoming_edges(reference, formula.find_references());
                    let cell = self.get_guaranteed(reference);
                    cell.input = input;
                    cell.formula = Some(formula);
                }
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
        self.get(reference)
            .and_then(|cell| cell.formula.as_ref())
            .map(|formula| evaluate(&formula.expr, &self.cells))
    }
}
