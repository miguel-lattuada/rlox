use crate::ast::expr::{Expr, Visitor};
use crate::ast::token::Token;
use crate::ast::tokentype::Literal;
use crate::error::RuntimeError;

pub struct AstPrinter;
impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        if let Ok(value) = expr.accept(self) {
            return value;
        }

        "".to_string()
    }

    fn parenthesize(&mut self, name: &String, expr: Vec<&Expr>) -> Result<String, RuntimeError> {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for e in expr {
            result.push(' ');

            if let Ok(value) = e.accept(self) {
                result.push_str(value.as_str());
            }
        }
        result.push(')');
        Ok(result)
    }
}
impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, RuntimeError> {
        self.parenthesize(&operator.lexeme, vec![left, right])
    }

    fn visit_literal_expr(&mut self, literal: &Literal) -> Result<String, RuntimeError> {
        let literal_string = match literal {
            Literal::String(ref s) => format!("\"{}\"", s),
            Literal::Number(ref n) => n.to_string(),
            Literal::Nil => "nil".to_string(),
            Literal::Boolean(ref b) => b.to_string(),
        };

        Ok(literal_string)
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<String, RuntimeError> {
        if let Expr::GroupingExpr(ref expression) = expr {
            self.parenthesize(&"group".to_string(), vec![expression])
        } else {
            panic!("Expected GroupingExpr")
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, RuntimeError> {
        self.parenthesize(&operator.lexeme, vec![right])
    }

    fn visit_variable_expr(&mut self, identifier: &Token) -> Result<String, RuntimeError> {
        todo!()
    }

    fn visit_assign_expr(
        &mut self,
        identifier: &Token,
        value: &Expr,
    ) -> Result<String, RuntimeError> {
        todo!()
    }
}
