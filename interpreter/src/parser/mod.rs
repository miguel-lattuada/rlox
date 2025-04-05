mod parser;
mod printer;

pub use parser::Parser;
pub use printer::AstPrinter;

pub fn example() {
    use crate::ast::{
        expr::{bexpr, gexpr, lexpr, uexpr},
        token::Token,
        tokentype::{Literal, TokenType},
    };

    let expression = bexpr(
        uexpr(
            Token::new(TokenType::Minus, "-", None, 1),
            lexpr(Literal::Number(123.0)),
        ),
        Token::new(TokenType::Star, "*", None, 1),
        gexpr(lexpr(Literal::Number(45.67))),
    );

    println!("{}", AstPrinter {}.print(&expression));
}
