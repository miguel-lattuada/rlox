use crate::ast::token::Token;
use crate::error::RuntimeError;

use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<T, RuntimeError>;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<T, RuntimeError>;
    fn visit_var_declaration_stmt(
        &mut self,
        identifier: &Token,
        initializer: Option<&Expr>,
    ) -> Result<T, RuntimeError>;
    fn visit_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<T, RuntimeError>;
    fn visit_if_stmt(
        &mut self,
        expr: &Expr,
        stmt_then: &Stmt,
        stmt_else: &Option<Box<Stmt>>,
    ) -> Result<T, RuntimeError>;
    fn visit_while_stmt(&mut self, expr: &Expr, stmt: &Stmt) -> Result<T, RuntimeError>;
    fn visit_function_stmt(
        &mut self,
        identifier: &Token,
        prameters: &Vec<Token>,
        body: &Box<Stmt>,
    ) -> Result<T, RuntimeError>;
    fn visit_return_stmt(&mut self, token: &Token, expr: &Expr) -> Result<T, RuntimeError>;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    VarDeclaration(Token, Option<Expr>),
    Function(Token, Vec<Token>, Box<Stmt>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Return(Token, Expr),
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
            VarDeclaration(ref identifier, ref initializer) => {
                visitor.visit_var_declaration_stmt(identifier, initializer.as_ref())
            }
            Block(ref stmts) => visitor.visit_block_stmt(stmts),
            If(ref expr, ref stmt_then, ref stmt_else) => {
                visitor.visit_if_stmt(expr, stmt_then, stmt_else)
            }
            While(ref expr, ref stmt) => visitor.visit_while_stmt(expr, stmt),
            Function(ref identifier, ref parameters, ref body) => {
                visitor.visit_function_stmt(identifier, parameters, body)
            }
            Return(ref token, ref expr) => visitor.visit_return_stmt(token, expr),
        }
    }
}

pub fn pstmt(expr: Expr) -> Stmt {
    Stmt::Print(expr)
}

pub fn estmt(expr: Expr) -> Stmt {
    Stmt::Expression(expr)
}

pub fn vdstmt(token: Token, initializer: Option<Expr>) -> Stmt {
    Stmt::VarDeclaration(token, initializer)
}

pub fn ifstmt(expr: Expr, stmt_then: Stmt, stmt_else: Option<Stmt>) -> Stmt {
    Stmt::If(expr, Box::new(stmt_then), stmt_else.map(Box::new))
}

pub fn wstmt(expr: Expr, stmt: Stmt) -> Stmt {
    Stmt::While(expr, Box::new(stmt))
}

pub fn fstmt(identifier: Token, parameters: Vec<Token>, body: Stmt) -> Stmt {
    Stmt::Function(identifier, parameters, Box::new(body))
}
