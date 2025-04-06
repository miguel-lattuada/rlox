use std::error::Error;

use super::object::Object;
use crate::ast::token::Token;
use crate::ast::tokentype::{Literal, TokenType};
use crate::{
    ast::expr::{Expr, Visitor},
    error::RuntimeError,
};

pub struct Interpreter;
impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        expr.accept(self)
    }

    fn non_numeric_operand_error<T>(&self) -> Result<T, RuntimeError> {
        Err(RuntimeError {
            message: "operands must be numeric for operation".to_string(),
        })
    }

    fn math_operation(
        &self,
        left_value: Object,
        right_value: Object,
        token_type: &TokenType,
    ) -> Result<Object, RuntimeError> {
        match (left_value, right_value) {
            (Object::Number(lvn), Object::Number(rvn)) => match token_type {
                TokenType::Plus => Ok(Object::Number(lvn + rvn)),
                TokenType::Minus => Ok(Object::Number(lvn - rvn)),
                TokenType::Star => Ok(Object::Number(lvn * rvn)),
                TokenType::Slash => Ok(Object::Number(lvn / rvn)),
                _ => Err(RuntimeError {
                    message: "unknown math operation".to_string(),
                }),
            },
            _ => self.non_numeric_operand_error(),
        }
    }
}

impl Visitor<Object> for Interpreter {
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
                self.math_operation(left_val, right_val, &operator.token_type)
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
                _ => self.non_numeric_operand_error(),
            },
            TokenType::GreaterEqual => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number >= right_number))
                }
                _ => self.non_numeric_operand_error(),
            },
            TokenType::Less => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number < right_number))
                }
                _ => self.non_numeric_operand_error(),
            },
            TokenType::LessEqual => match (left_val, right_val) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number <= right_number))
                }
                _ => self.non_numeric_operand_error(),
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
                _ => self.non_numeric_operand_error(),
            },
            TokenType::Bang => Ok(Object::Boolean(!bool::from(right_expr_value))),
            _ => Err(RuntimeError {
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
}
