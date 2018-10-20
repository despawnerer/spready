use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Integer(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
    ParseError,
    IncompatibleTypes,
    DivisionByZero,
}

pub type EvaluationResult = Result<Value, EvaluationError>;

impl Default for Value {
    fn default() -> Value {
        Value::None
    }
}

impl FromStr for Value {
    type Err = EvaluationError;

    fn from_str(text: &str) -> Result<Value, EvaluationError> {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f, ""),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Text(value) => write!(f, "{}", value),
        }
    }
}
