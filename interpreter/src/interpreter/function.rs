use std::fmt::Debug;

use crate::error::RuntimeError;

use super::{object::Object, Interpreter};

#[derive(Debug, Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: fn(&Vec<Object>) -> Object,
    },
    User,
}

impl Function {
    pub fn call(
        &self,
        _interpreter: &Interpreter,
        arguments: &Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        use Function::*;
        match self {
            Native { body, .. } => Ok(body(arguments)),
            _ => Ok(Object::Nil),
        }
    }

    pub fn arity(&self) -> usize {
        use Function::*;
        match self {
            Native { arity, .. } => *arity,
            _ => 0,
        }
    }
}
