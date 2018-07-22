use std::str::FromStr;
use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug, Clone)]
pub struct InvalidValue;

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Integer(i64),
    Float(f64),
    Text(String),
}

pub type MaybeValue = Result<Value, InvalidValue>;

impl Add for Value {
    type Output = MaybeValue;

    fn add(self, other: Value) -> MaybeValue {
        match (self, other) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x + y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
            _ => Err(InvalidValue),
        }
    }
}

impl Sub for Value {
    type Output = MaybeValue;

    fn sub(self, other: Value) -> MaybeValue {
        match (self, other) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x - y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
            _ => Err(InvalidValue),
        }
    }
}

impl Mul for Value {
    type Output = MaybeValue;

    fn mul(self, other: Value) -> MaybeValue {
        match (self, other) {
            (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x * y)),
            (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
            _ => Err(InvalidValue),
        }
    }
}

impl Div for Value {
    type Output = MaybeValue;

    fn div(self, other: Value) -> MaybeValue {
        match (self, other) {
            (Value::Integer(x), Value::Integer(y)) if y > 0 => Ok(Value::Integer(x / y)),
            (Value::Integer(_), Value::Integer(y)) if y == 0 => Err(InvalidValue),
            (Value::Float(x), Value::Float(y)) if y > 0.0 => Ok(Value::Float(x / y)),
            (Value::Float(_), Value::Float(y)) if y == 0.0 => Err(InvalidValue),
            _ => Err(InvalidValue),
        }
    }
}

impl Default for Value {
    fn default() -> Value {
        Value::None
    }
}

impl FromStr for Value {
    type Err = InvalidValue;

    fn from_str(text: &str) -> Result<Value, InvalidValue> {
        if text.trim().is_empty() {
            Ok(Value::None)
        } else if let Ok(value) = text.parse::<i64>() {
            Ok(Value::Integer(value))
        } else if let Ok(value) = text.parse::<f64>() {
            Ok(Value::Float(value))
        } else {
            Ok(Value::Text(text.to_owned()))
        }
    }
}
