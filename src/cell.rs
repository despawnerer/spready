use std::collections::HashSet;

use reference::Reference;
use value::{InvalidValue, Value};

#[derive(Clone, Debug)]
pub struct Cell {
    pub input: String,
    pub value: Result<Value, InvalidValue>,
    pub successors: HashSet<Reference>,
}

impl Cell {
    pub fn new(input: String, value: Result<Value, InvalidValue>) -> Cell {
        Cell {
            input,
            value,
            successors: HashSet::new(),
        }
    }
}
