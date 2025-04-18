use super::object::Object;
use crate::ast::token::Token;
use crate::error::RuntimeError;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: &Token, value: Option<Object>) {
        self.values.insert(identifier.lexeme.clone(), value);
    }

    pub fn assign(
        &mut self,
        identifier: &Token,
        value: Option<Object>,
    ) -> Result<Object, RuntimeError> {
        match self.values.get(&identifier.lexeme) {
            Some(old_value) => {
                self.values.insert(identifier.lexeme.clone(), value.clone());
                Ok(value.unwrap())
            }
            None => self.undefined(identifier.clone()),
        }
    }

    pub fn get(&self, identifier: &Token) -> Result<Object, RuntimeError> {
        match self.values.get(&identifier.lexeme) {
            Some(value) => {
                if let Some(value) = value {
                    return Ok(value.clone());
                }
                self.uninitialized(identifier.clone())
            }
            None => self.undefined(identifier.clone()),
        }
    }

    fn uninitialized(&self, token: Token) -> Result<Object, RuntimeError> {
        Err(RuntimeError {
            message: format!("Uninitialized variable '{}'.", token.lexeme),
            token,
        })
    }

    fn undefined(&self, token: Token) -> Result<Object, RuntimeError> {
        Err(RuntimeError {
            message: format!("Undefined variable '{}'.", token.lexeme),
            token,
        })
    }
}
