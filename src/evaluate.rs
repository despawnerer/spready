use cell::Sheet;
use syntax::{Expr, Opcode};
use value::{MaybeValue, Value};

pub fn evaluate(expr: &Expr, cells: &Sheet) -> MaybeValue {
    match expr {
        Expr::Integer(x) => Ok(Value::Integer(*x)),
        Expr::Float(x) => Ok(Value::Float(*x)),
        Expr::Reference(x) => match cells.get(&x) {
            Some(cell) => cell.value.clone(),
            None => Ok(Value::None),
        },
        Expr::Op(l, op, r) => {
            let l = evaluate(l, cells)?;
            let r = evaluate(r, cells)?;

            let (l, r) = coerce_types(l, r);

            match op {
                Opcode::Mul => l * r,
                Opcode::Div => l / r,
                Opcode::Add => l + r,
                Opcode::Sub => l - r,
            }
        }
    }
}

fn coerce_types(x: Value, y: Value) -> (Value, Value) {
    match (x, y) {
        (x, Value::None) => coerce_types(x, Value::Integer(0)),
        (Value::None, y) => coerce_types(Value::Integer(0), y),
        (Value::Integer(x), Value::Float(y)) => (Value::Float(x as f64), Value::Float(y)),
        (Value::Float(x), Value::Integer(y)) => (Value::Float(x), Value::Float(y as f64)),
        otherwise => otherwise,
    }
}
