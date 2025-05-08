use super::object::Object;
use crate::ast::token::Token;
use crate::error::RuntimeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Option<Object>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
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
            Some(_old_value) => {
                self.values.insert(identifier.lexeme.clone(), value.clone());
                Ok(value.unwrap())
            }
            None => {
                if let Some(ref env) = self.enclosing {
                    return env.borrow_mut().assign(identifier, value);
                }

                self.undefined(identifier.clone())
            }
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
            None => {
                if let Some(ref env) = self.enclosing {
                    return env.borrow().get(identifier);
                }
                self.undefined(identifier.clone())
            }
        }
    }

    fn uninitialized(&self, token: Token) -> Result<Object, RuntimeError> {
        Err(RuntimeError {
            value: None,
            message: format!("Uninitialized variable '{}'.", token.lexeme),
            token,
        })
    }

    fn undefined(&self, token: Token) -> Result<Object, RuntimeError> {
        Err(RuntimeError {
            value: None,
            message: format!("Undefined variable '{}'.", token.lexeme),
            token,
        })
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = self.values.iter().collect::<Vec<_>>();

        write!(f, "Current: {:?} - Parent: {:?}", elements, self.enclosing)
    }
}
