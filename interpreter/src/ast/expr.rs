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
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(Box<Expr>, Token, Box<Expr>),
    GroupingExpr(Box<Expr>),
    LiteralExpr(Literal),
    UnaryExpr(Token, Box<Expr>),
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
            GroupingExpr(ref expression) => visitor.visit_grouping_expr(self),
            UnaryExpr(ref operator, ref expression) => {
                visitor.visit_unary_expr(operator, expression)
            }
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
