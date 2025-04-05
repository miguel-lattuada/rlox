use std::error::Error;

use super::token::Token;
use super::tokentype::Literal;

pub trait Visitor<T> {
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}

pub enum Expr {
    BinaryExpr(Box<Expr>, Token, Box<Expr>),
    GroupingExpr(Box<Expr>),
    LiteralExpr(Literal),
    UnaryExpr(Token, Box<Expr>),
}

impl Expr {
    pub fn accept<T, U>(&self, visitor: &mut U) -> T
    where
        U: Visitor<T>,
    {
        use Expr::*;

        match *self {
            LiteralExpr(ref literal) => visitor.visit_literal_expr(self),
            BinaryExpr(ref left, ref operator, ref right) => visitor.visit_binary_expr(self),
            GroupingExpr(ref expression) => visitor.visit_grouping_expr(self),
            UnaryExpr(ref operator, ref expression) => visitor.visit_unary_expr(self),
            _ => {
                panic!("Unknown expression type");
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
