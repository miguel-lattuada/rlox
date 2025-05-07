use super::token::Token;
use super::tokentype::Literal;
use crate::error::RuntimeError;

pub trait Visitor<T> {
    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<T, RuntimeError>;
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<T, RuntimeError>;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<T, RuntimeError>;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<T, RuntimeError>;
    fn visit_variable_expr(&mut self, identifier: &Token) -> Result<T, RuntimeError>;
    fn visit_assign_expr(&mut self, identifier: &Token, value: &Expr) -> Result<T, RuntimeError>;
    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<T, RuntimeError>;
    fn visit_call_expr(
        &mut self,
        calee: &Expr,
        paren: &Token,
        args: &Vec<Expr>,
    ) -> Result<T, RuntimeError>;
}

#[derive(Debug, Clone)]
pub enum Expr {
    // TODO: Remove Expr postfix
    AssignExpr(Token, Box<Expr>),
    BinaryExpr(Box<Expr>, Token, Box<Expr>),
    GroupingExpr(Box<Expr>),
    LiteralExpr(Literal),
    UnaryExpr(Token, Box<Expr>),
    VariableExpr(Token),
    LogicalExpr(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

impl Expr {
    pub fn accept<T, U>(&self, visitor: &mut U) -> Result<T, RuntimeError>
    where
        U: Visitor<T>,
    {
        use Expr::*;

        match *self {
            LiteralExpr(ref literal) => visitor.visit_literal_expr(literal),
            BinaryExpr(ref left, ref operator, ref right) => {
                visitor.visit_binary_expr(left, operator, right)
            }
            GroupingExpr(ref _expression) => visitor.visit_grouping_expr(self),
            UnaryExpr(ref operator, ref expression) => {
                visitor.visit_unary_expr(operator, expression)
            }
            VariableExpr(ref token) => visitor.visit_variable_expr(token),
            AssignExpr(ref token, ref expr) => visitor.visit_assign_expr(token, expr),
            LogicalExpr(ref left, ref operator, ref right) => {
                visitor.visit_logical_expr(left, operator, right)
            }
            Call(ref callee, ref paren, ref args) => visitor.visit_call_expr(callee, paren, args),
        }
    }
}

pub fn bexpr(left: Expr, operator: Token, right: Expr) -> Expr {
    Expr::BinaryExpr(Box::new(left), operator, Box::new(right))
}

pub fn gexpr(group: Expr) -> Expr {
    Expr::GroupingExpr(Box::new(group))
}

pub fn lexpr(literal: Literal) -> Expr {
    Expr::LiteralExpr(literal)
}

pub fn uexpr(operator: Token, right: Expr) -> Expr {
    Expr::UnaryExpr(operator, Box::new(right))
}

pub fn vexpr(identifier: Token) -> Expr {
    Expr::VariableExpr(identifier)
}

pub fn aexpr(identifier: Token, value: Expr) -> Expr {
    Expr::AssignExpr(identifier, Box::new(value))
}

pub fn lgexpr(left: Expr, operator: Token, right: Expr) -> Expr {
    Expr::LogicalExpr(Box::new(left), operator, Box::new(right))
}

pub fn cexpr(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Expr {
    Expr::Call(Box::new(callee), paren, arguments)
}
