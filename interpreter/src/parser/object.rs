use crate::error::RuntimeError;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl From<Object> for bool {
    fn from(object: Object) -> Self {
        match object {
            Object::Boolean(boolean) => boolean,
            Object::Nil => false,
            _ => true,
        }
    }
}

impl From<Object> for String {
    fn from(object: Object) -> Self {
        match object {
            Object::Number(number) => number.to_string(),
            Object::Boolean(boolean) => boolean.to_string(),
            Object::String(string) => string,
            Object::Nil => "nil".to_string(),
        }
    }
}

impl TryFrom<Object> for f64 {
    type Error = RuntimeError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Number(number) => Ok(number),
            Object::Boolean(boolean) => {
                if boolean {
                    Ok(1.0)
                } else {
                    Ok(0.0)
                }
            }
            Object::Nil => Ok(0.0),
            _ => Err(RuntimeError {
                message: format!("cannot convert [{:?}] to a number", value),
            }),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(l), Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}
