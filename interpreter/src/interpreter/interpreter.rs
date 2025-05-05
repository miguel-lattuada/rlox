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
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter<'a> {
    env: Rc<RefCell<Environment>>,
    _reporter: Option<&'a ErrorReporter>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new(None))),
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

    fn execute_block(&mut self, stmts: &Vec<Stmt>, env: Environment) -> Result<(), RuntimeError> {
        let prev_env = Rc::clone(&self.env);

        self.env = Rc::new(RefCell::new(env));

        for stmt in stmts {
            if let Ok(()) = self.execute(stmt) {
                continue;
            } else {
                break;
            }
        }

        self.env = prev_env;

        Ok(())
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
        self.env.borrow().get(identifier)
    }

    fn visit_assign_expr(
        &mut self,
        identifier: &Token,
        value: &Expr,
    ) -> Result<Object, RuntimeError> {
        let val = self.evaluate(value)?;
        self.env.borrow_mut().assign(identifier, Some(val))
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, RuntimeError> {
        let left = self.evaluate(left)?;
        let boolean_value = bool::from(&left);

        if (operator.token_type == TokenType::Or && boolean_value)
            || (operator.token_type == TokenType::And && !boolean_value)
        {
            return Ok(left);
        }

        return self.evaluate(right);
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

        // self.env.define(identifier, value);
        self.env.borrow_mut().define(identifier, value);

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), RuntimeError> {
        let clone = Rc::clone(&self.env);
        self.execute_block(stmts, Environment::new(Some(clone)))?;
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        expr: &Expr,
        stmt_then: &Stmt,
        stmt_else: &Option<Box<Stmt>>,
    ) -> Result<(), RuntimeError> {
        let condition_result = self.evaluate(expr)?;
        let boolean_result = bool::from(condition_result);

        if boolean_result {
            self.execute(stmt_then)?;
        } else if let Some(_else) = stmt_else {
            self.execute(_else)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<(), RuntimeError> {
        while bool::from(self.evaluate(expr)?) {
            self.execute(stmt)?;
        }
        Ok(())
    }
}
