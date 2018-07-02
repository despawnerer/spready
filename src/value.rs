use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct InvalidValue;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Text(String),
}

pub type MaybeValue = Result<Value, InvalidValue>;

impl FromStr for Value {
    type Err = InvalidValue;

    fn from_str(text: &str) -> MaybeValue {
        if let Ok(value) = text.parse::<i64>() {
            Ok(Value::Integer(value))
        } else if let Ok(value) = text.parse::<f64>() {
            Ok(Value::Float(value))
        } else {
            Ok(Value::Text(text.to_owned()))
        }
    }
}
