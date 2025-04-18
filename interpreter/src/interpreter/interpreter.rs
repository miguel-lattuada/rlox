use super::environment::Environment;
use super::object::Object;
use crate::ast::token::Token;
use crate::ast::tokentype::{Literal, TokenType};
use crate::error::ErrorReporter;
use crate::{
    ast::expr::{Expr, Visitor as ExprVisitor},
    ast::stmt::{Stmt, Visitor as StmtVisitor},
    error::RuntimeError,
};
use std::collections::HashMap;

pub struct Interpreter<'a> {
    env: Environment,
    _reporter: Option<&'a ErrorReporter>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            _reporter: None,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(&stmt)
                .inspect_err(|e| {
                    self.error(&e.token, e.message.as_str());
                })
                .unwrap();
        }
    }

    pub fn set_error_reporter(&mut self, reporter: &'a ErrorReporter) {
        self._reporter = Some(reporter);
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn non_numeric_operand_error<T>(&self, token: &Token) -> Result<T, RuntimeError> {
        Err(RuntimeError {
            token: token.clone(),
            message: "operands must be numeric for operation".to_string(),
        })
    }

    fn math_operation(
        &self,
        left_value: Object,
        right_value: Object,
        token: &Token,
    ) -> Result<Object, RuntimeError> {
        match (left_value, right_value) {
            (Object::Number(lvn), Object::Number(rvn)) => match token.token_type {
                TokenType::Plus => Ok(Object::Number(lvn + rvn)),
                TokenType::Minus => Ok(Object::Number(lvn - rvn)),
                TokenType::Star => Ok(Object::Number(lvn * rvn)),
                TokenType::Slash => Ok(Object::Number(lvn / rvn)),
                _ => Err(RuntimeError {
                    token: token.clone(),
                    message: "unknown math operation".to_string(),
                }),
            },
            _ => self.non_numeric_operand_error(token),
        }
    }

    fn error(&self, token: &Token, message: &str) {
        match self._reporter {
            Some(reporter) => reporter.runtime_error(token, message),

            // Reporter does not exist, print to stderr
            None => eprintln!("[Error]: {}", message),
        }
    }
}

impl<'a> ExprVisitor<Object> for Interpreter<'a> {
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, RuntimeError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus | TokenType::Star | TokenType::Slash => {
                self.math_operation(left_val, right_val, &operator)
            }
            TokenType::Plus => match (&left_val, &right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number + right_number))
                }
                _ => {
                    // DECISION #1: convert the operands to string if they are not number
                    Ok(Object::String(
                        String::from(left_val.clone()) + &String::from(right_val.clone()),
                    ))
                }
            },
            TokenType::Greater => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number > right_number))
                }
                _ => self.non_numeric_operand_error(&operator),
            },
            TokenType::GreaterEqual => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number >= right_number))
                }
                _ => self.non_numeric_operand_error(&operator),
            },
            TokenType::Less => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number < right_number))
                }
                _ => self.non_numeric_operand_error(&operator),
            },
            TokenType::LessEqual => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number <= right_number))
                }
                _ => self.non_numeric_operand_error(&operator),
            },
            TokenType::BangEqual => Ok(Object::Boolean(left_val != right_val)),
            TokenType::EqualEqual => Ok(Object::Boolean(left_val == right_val)),
            _ => {
                todo!()
            }
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, RuntimeError> {
        let right_expr_value = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match right_expr_value {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => self.non_numeric_operand_error(&operator),
            },
            TokenType::Bang => Ok(Object::Boolean(!bool::from(right_expr_value))),
            _ => Err(RuntimeError {
                token: operator.clone(),
                message: "unexpected token on unary expression".to_string(),
            }),
        }
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<Object, RuntimeError> {
        let literal = match literal {
            Literal::String(ref s) => Object::String(s.clone()),
            Literal::Number(ref n) => Object::Number(*n),
            Literal::Nil => Object::Nil,
            Literal::Boolean(ref b) => Object::Boolean(*b),
        };
        return Ok(literal);
    }

    fn visit_variable_expr(&mut self, identifier: &Token) -> Result<Object, RuntimeError> {
        self.env.get(identifier)
    }

    fn visit_assign_expr(
        &mut self,
        identifier: &Token,
        value: &Expr,
    ) -> Result<Object, RuntimeError> {
        let val = self.evaluate(value)?;
        self.env.assign(identifier, Some(val))
    }
}

impl<'a> StmtVisitor<()> for Interpreter<'a> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        let value = self.evaluate(expr)?;
        // TODO: implement Display on Object
        println!("{:?}", value);
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
        self.evaluate(expr)?;
        Ok(())
    }

    fn visit_var_declaration_stmt(
        &mut self,
        identifier: &Token,
        initializer: Option<&Expr>,
    ) -> Result<(), RuntimeError> {
        let mut value = None;

        if let Some(expr) = initializer {
            value = Some(self.evaluate(expr)?);
        }

        self.env.define(identifier, value);

        Ok(())
    }
}
