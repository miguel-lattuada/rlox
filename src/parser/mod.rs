mod expr;
mod printer;

pub fn example() {
    use crate::scanner::{Literal, Token, TokenType};

    use expr::{bexpr, gexpr, lexpr, uexpr};
    use printer::AstPrinter;

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
