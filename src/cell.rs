use std::collections::BTreeMap;

use crate::formula::Expr;
use crate::reference::Reference;
use crate::value::{EvaluationError, EvaluationResult, Value};

pub type Sheet = BTreeMap<Reference, Cell>;

#[derive(Clone, Debug)]
pub struct Cell {
    pub content: Content,
    pub value: EvaluationResult,
}

#[derive(Clone, Debug)]
pub enum Content {
    Text(String),
    Formula {
        text: String,
        formula: Result<Expr, EvaluationError>,
    },
}

impl Cell {
    pub fn text(&self) -> &str {
        match self.content {
            Content::Text(ref text) => text.as_str(),
            Content::Formula {
                ref text,
                formula: _,
            } => text.as_str(),
        }
    }

    pub fn formula(&self) -> Option<&Expr> {
        match self.content {
            Content::Formula {
                text: _,
                formula: Ok(ref formula),
            } => Some(formula),
            _ => None,
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            content: Content::Text("".into()),
            value: Ok(Value::None),
        }
    }
}
