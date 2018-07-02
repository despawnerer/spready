use std::collections::HashMap;

use reference::Reference;
use value::{Value, InvalidValue};
use cell::Cell;

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
			Expr::Reference(x) => cells.get(&x).unwrap().value.clone(),
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
