use std::str::FromStr;

use crate::cell::{Cell, Content, Sheet};
use crate::evaluate::evaluate;
use crate::formula::Formula;
use crate::graph::DirectedGraph;
use crate::reference::Reference;
use crate::value::EvaluationResult;

#[derive(Debug, Default)]
pub struct Spreadsheet {
    cells: Sheet,
    dependencies: DirectedGraph,
}

impl Spreadsheet {
    pub fn new() -> Spreadsheet {
        Default::default()
    }

    pub fn cells(&self) -> &Sheet {
        &self.cells
    }

    pub fn dependencies(&self) -> &DirectedGraph {
        &self.dependencies
    }

    pub fn get<R: Into<Reference>>(&self, reference: R) -> Option<&Cell> {
        self.cells.get(&reference.into())
    }

    pub fn get_guaranteed_mut<R: Into<Reference>>(&mut self, reference: R) -> &mut Cell {
        self.cells
            .entry(reference.into())
            .or_insert_with(Cell::default)
    }

    pub fn enter<R, T>(&mut self, reference: R, text: T)
    where
        R: Into<Reference>,
        T: ToString,
    {
        let reference = reference.into();
        let text = text.to_string();

        if text.starts_with('=') {
            match Formula::from_str(&text) {
                Ok(formula) => {
                    self.dependencies
                        .set_incoming_edges(reference, formula.find_references());
                    let cell = self.get_guaranteed_mut(reference);
                    cell.content = Content::Formula {
                        text: text,
                        formula: Ok(formula),
                    };
                }
                Err(error) => {
                    let cell = self.get_guaranteed_mut(reference);
                    cell.content = Content::Formula {
                        text: text,
                        formula: Err(error),
                    };
                }
            }
        } else {
            let cell = self.get_guaranteed_mut(reference);
            cell.value = text.parse();
            cell.content = Content::Text(text);
        }

        self.recalculate();
    }

    fn recalculate(&mut self) {
        let order = self.dependencies.to_topological_sort().unwrap();
        for reference in order.iter().cloned() {
            if let Some(value) = self.calculate_cell(reference) {
                self.get_guaranteed_mut(reference).value = value
            }
        }
    }

    fn calculate_cell(&self, reference: Reference) -> Option<EvaluationResult> {
        self.get(reference)
            .and_then(|cell| cell.formula())
            .map(|formula| evaluate(&formula.expr, &self.cells))
    }
}
