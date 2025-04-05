use crate::ast::expr::{Expr, Visitor};
use crate::ast::tokentype::Literal;

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &String, expr: Vec<&Expr>) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for e in expr {
            result.push(' ');
            result.push_str(e.accept(self).as_str());
        }
        result.push(')');
        result
    }
}
impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::BinaryExpr(ref left, ref operator, ref right) = expr {
            self.parenthesize(&operator.lexeme, vec![left, right])
        } else {
            panic!("Expected BinaryExpr")
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        if let Expr::LiteralExpr(ref literal) = expr {
            match literal {
                Literal::String(ref s) => format!("\"{}\"", s),
                Literal::Number(ref n) => n.to_string(),
                Literal::Nil => "nil".to_string(),
                Literal::Boolean(ref b) => b.to_string(),
            }
        } else {
            panic!("Expected LiteralExpr")
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        if let Expr::GroupingExpr(ref expression) = expr {
            self.parenthesize(&"group".to_string(), vec![expression])
        } else {
            panic!("Expected GroupingExpr")
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::UnaryExpr(ref operator, ref expression) = expr {
            self.parenthesize(&operator.lexeme, vec![expression])
        } else {
            panic!("Expected UnaryExpr")
        }
    }
}
