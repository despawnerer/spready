use crate::cell::Sheet;
use crate::parser::{self, BinaryOp, Expr};
use crate::value::Value::{Float, Integer};
use crate::value::{EvaluationError, EvaluationResult, Value};

pub fn evaluate(expr: &Expr, cells: &Sheet) -> EvaluationResult {
    match expr {
        Expr::Value(parser::Value::Integer(x)) => Ok(Integer(*x)),
        Expr::Value(parser::Value::Float(x)) => Ok(Float(*x)),
        Expr::Reference(x) => match cells.get(&x) {
            Some(cell) => cell.value.clone(),
            None => Ok(Value::None),
        },
        Expr::Op(l, op, r) => {
            let l = evaluate(&l.0, cells)?;
            let r = evaluate(&r.0, cells)?;

            let (l, r) = coerce_types(l, r);

            match op {
                BinaryOp::Mul => mul(l, r),
                BinaryOp::Div => div(l, r),
                BinaryOp::Add => add(l, r),
                BinaryOp::Sub => sub(l, r),
            }
        }
    }
}

fn coerce_types(x: Value, y: Value) -> (Value, Value) {
    match (x, y) {
        (x, Value::None) => coerce_types(x, Integer(0)),
        (Value::None, y) => coerce_types(Integer(0), y),
        (Integer(x), Float(y)) => (Float(x as f64), Float(y)),
        (Float(x), Integer(y)) => (Float(x), Float(y as f64)),
        otherwise => otherwise,
    }
}

fn add(x: Value, y: Value) -> EvaluationResult {
    match (x, y) {
        (Integer(x), Integer(y)) => Ok(Integer(x + y)),
        (Float(x), Float(y)) => Ok(Float(x + y)),
        _ => Err(EvaluationError::IncompatibleTypes),
    }
}

fn sub(x: Value, y: Value) -> EvaluationResult {
    match (x, y) {
        (Integer(x), Integer(y)) => Ok(Integer(x - y)),
        (Float(x), Float(y)) => Ok(Float(x - y)),
        _ => Err(EvaluationError::IncompatibleTypes),
    }
}

fn mul(x: Value, y: Value) -> EvaluationResult {
    match (x, y) {
        (Integer(x), Integer(y)) => Ok(Integer(x * y)),
        (Float(x), Float(y)) => Ok(Float(x * y)),
        _ => Err(EvaluationError::IncompatibleTypes),
    }
}

fn div(x: Value, y: Value) -> EvaluationResult {
    match (x, y) {
        (Integer(x), Integer(y)) if y > 0 => Ok(Integer(x / y)),
        (Integer(_), Integer(y)) if y == 0 => Err(EvaluationError::DivisionByZero),
        (Float(x), Float(y)) if y > 0.0 => Ok(Float(x / y)),
        (Float(_), Float(y)) if y == 0.0 => Err(EvaluationError::DivisionByZero),
        _ => Err(EvaluationError::IncompatibleTypes),
    }
}
