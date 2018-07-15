use std::str::FromStr;

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
