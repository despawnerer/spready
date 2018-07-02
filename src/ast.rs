use std::collections::HashMap;

use cell::Cell;
use reference::Reference;
use value::{InvalidValue, Value};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Reference(Reference),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Expr {
    pub fn evaluate(self, cells: &HashMap<Reference, Cell>) -> Result<Value, InvalidValue> {
        match self {
            Expr::Number(x) => Ok(Value::Integer(x)),
            Expr::Reference(x) => match cells.get(&x) {
                Some(cell) => cell.value.clone(),
                None => Ok(Value::Integer(0)),
            },
            Expr::Op(l, op, r) => {
                let l = match (*l).evaluate(cells) {
                    Ok(Value::Integer(x)) => x,
                    _ => return Err(InvalidValue),
                };

                let r = match (*r).evaluate(cells) {
                    Ok(Value::Integer(x)) => x,
                    _ => return Err(InvalidValue),
                };

                let result = match op {
                    Opcode::Mul => l * r,
                    Opcode::Div => l / r,
                    Opcode::Add => l + r,
                    Opcode::Sub => l - r,
                };

                Ok(Value::Integer(result))
            }
        }
    }
}
