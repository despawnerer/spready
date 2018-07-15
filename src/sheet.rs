use std::collections::BTreeMap;

use formula::Formula;
use reference::Reference;
use value::{MaybeValue, Value};

pub type Sheet = BTreeMap<Reference, Cell>;

#[derive(Clone, Debug)]
pub struct Cell {
    pub input: String,
    pub value: MaybeValue,
    pub formula: Option<Formula>,
}

impl Cell {
    pub fn new(input: String, value: MaybeValue, formula: Option<Formula>) -> Cell {
        Cell {
            input,
            value,
            formula,
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new("".into(), Ok(Value::None), None)
    }
}
