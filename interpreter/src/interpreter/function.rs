use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    ast::{stmt::Stmt, token::Token},
    error::RuntimeError,
    interpreter::environment::Environment,
};

use super::{object::Object, Interpreter};

#[derive(Debug, Clone)]
pub enum Function {
    Native {
        identifier: String,
        arity: usize,
        body: fn(&Vec<Object>) -> Object,
    },
    User {
        identifier: Token,
        parameters: Vec<Token>,
        body: Box<Stmt>,
    },
}

impl Function {
    pub fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        use Function::*;

        match self {
            Native { body, .. } => Ok(body(arguments)),
            User {
                body,
                identifier,
                parameters,
            } => match **body {
                Stmt::Block(ref stmts) => {
                    let mut env = Environment::new(Some(Rc::clone(&_interpreter.globals)));

                    for (idx, token) in parameters.iter().enumerate() {
                        env.define(token, arguments.get(idx).cloned());
                    }

                    if let Err(err) = _interpreter.execute_block(stmts, env) {
                        return Ok(err.value.unwrap());
                    }

                    Ok(Object::Nil)
                }
                _ => Err(RuntimeError {
                    value: None,
                    token: identifier.clone(),
                    message: "[UNREACHABLE] Function statements must be a block.".to_string(),
                }),
            },
        }
    }

    pub fn arity(&self) -> usize {
        use Function::*;
        match self {
            Native { arity, .. } => *arity,
            User { parameters, .. } => parameters.len(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Function::*;
        match self {
            Native { identifier, .. } => write!(f, "<native fn {}>", identifier),
            User { identifier, .. } => write!(f, "<fn {}>", identifier.lexeme),
        }
    }
}
