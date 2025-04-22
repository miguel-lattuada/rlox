use std::fmt::Debug;

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
