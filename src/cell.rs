use std::collections::HashSet;

use value::{Value, InvalidValue};
use reference::Reference;

#[derive(Clone, Debug)]
pub struct Cell {
    pub input: String,
    pub value: Result<Value, InvalidValue>,
    pub successors: HashSet<Reference>,
}

impl Cell {
    pub fn new(input: String, value: Result<Value, InvalidValue>) -> Cell {
        Cell {
            input, value, successors: HashSet::new()
        }
    }
}
