mod parser;

pub use parser::Parser;

fn _example() {
    use crate::ast::{
        expr::{bexpr, gexpr, lexpr, uexpr},
        token::Token,
        tokentype::{Literal, TokenType},
    };

    let _expression = bexpr(
        uexpr(
            Token::new(TokenType::Minus, "-", None, 1),
            lexpr(Literal::Number(123.0)),
        ),
        Token::new(TokenType::Star, "*", None, 1),
        gexpr(lexpr(Literal::Number(45.67))),
    );
}
