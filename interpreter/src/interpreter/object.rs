use std::fmt::{Debug, Display};

use super::function::Function;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Function),
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

impl From<&Object> for bool {
    fn from(object: &Object) -> Self {
        match object {
            Object::Boolean(boolean) => *boolean,
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
            Object::Callable(_fn) => "<native fn>".to_string(),
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

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Callable(ref fun) => write!(f, "{}", fun),
            _ => write!(f, "{:?}", self),
        }
    }
}
