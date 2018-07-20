use sheet::Sheet;
use syntax::{Expr, Opcode};
use value::{InvalidValue, MaybeValue, Value};

pub fn evaluate(expr: &Expr, cells: &Sheet) -> MaybeValue {
    match expr {
        Expr::Integer(x) => Ok(Value::Integer(*x)),
        Expr::Reference(x) => match cells.get(&x) {
            Some(cell) => match cell.value {
                Ok(Value::None) => Ok(Value::Integer(0)),
                _ => cell.value.clone(),
            },
            None => Ok(Value::Integer(0)),
        },
        Expr::Op(l, op, r) => {
            let l = evaluate_expecting_integer(l, cells)?;
            let r = evaluate_expecting_integer(r, cells)?;

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

pub fn evaluate_expecting_integer(expr: &Expr, cells: &Sheet) -> Result<i64, InvalidValue> {
    match evaluate(expr, cells) {
        Ok(Value::Integer(value)) => Ok(value),
        Ok(_) => Err(InvalidValue),
        Err(error) => Err(error),
    }
}
