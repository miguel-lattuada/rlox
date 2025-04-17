use crate::error::RuntimeError;

use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<T, RuntimeError>;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<T, RuntimeError>;
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
}

impl Stmt {
    pub fn accept<T, U>(&self, visitor: &mut U) -> Result<T, RuntimeError>
    where
        U: Visitor<T>,
    {
        use Stmt::*;
        match *self {
            Print(ref expr) => visitor.visit_print_stmt(expr),
            Expression(ref expr) => visitor.visit_expression_stmt(expr),
        }
    }
}

pub fn pstmt(expr: Expr) -> Stmt {
    Stmt::Print(expr)
}

pub fn estmt(expr: Expr) -> Stmt {
    Stmt::Expression(expr)
}
