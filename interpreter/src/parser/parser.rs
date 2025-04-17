use std::cell::Cell;

use crate::{
    ast::{
        expr::{bexpr, gexpr, lexpr, uexpr, Expr},
        stmt::{estmt, pstmt, Stmt},
        token::Token,
        tokentype::{Literal, TokenType},
    },
    error::{ErrorReporter, ParseError},
};

pub struct Parser<'a> {
    _current: Cell<usize>,
    _reporter: Option<&'a ErrorReporter>,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self {
            _reporter: None,
            _current: Cell::new(0),
            tokens,
        };
    }

    pub fn set_error_reporter(&mut self, reporter: &'a ErrorReporter) {
        self._reporter = Some(reporter);
    }

    pub fn parse(&self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let stmt = self
                .statement()
                .inspect_err(|e| {
                    self.error(&e.token, e.message.as_str());
                })
                .unwrap();
            statements.push(stmt);
        }

        statements
    }

    /**
    * Parse grammar rule: statement      → exprStmt
                                           | printStmt ;
    */
    fn statement(&self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenType::Print]) {
            return self.print_stmt();
        }

        self.expression_stmt()
    }

    /**
     * Parse grammar rule: printStmt      → "print" expression ";" ;
     */
    fn print_stmt(&self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(pstmt(value))
    }

    /**
     * Parse grammar rule: exprStmt       → expression ";" ;
     */
    fn expression_stmt(&self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(estmt(expr))
    }

    /**
     * Parse grammar rule: expression    → equality
     */
    fn expression(&self) -> Result<Expr, ParseError> {
        self.equality()
    }

    /**
     * Parse grammar rule: equality       → comparison ( ( "!=" | "==" ) comparison )* ;
     */
    fn equality(&self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = bexpr(expr, operator.clone(), right);
        }
        Ok(expr)
    }

    /**
     * Parse grammar rule: comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
     */
    fn comparison(&self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = bexpr(expr, operator.clone(), right)
        }
        Ok(expr)
    }

    /**
     * Parse grammar rule: term           → factor ( ( "-" | "+" ) factor )* ;
     */
    fn term(&self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = bexpr(expr, operator.clone(), right);
        }
        Ok(expr)
    }

    /**
     * Parse grammar rule: factor         → unary ( ( "/" | "*" ) unary )* ;
     */
    fn factor(&self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = bexpr(expr, operator.clone(), right);
        }
        Ok(expr)
    }

    /**
     * Parse grammar rule: unary          → ( "!" | "-" ) unary
     *                                      | primary ;
     */
    fn unary(&self) -> Result<Expr, ParseError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(uexpr(operator.clone(), right));
        }
        self.primary()
    }

    /**
     * Parse grammer rule: primary        → NUMBER | STRING | "true" | "false" | "nil"
     *                                      | "(" expression ")" ;
     */
    fn primary(&self) -> Result<Expr, ParseError> {
        if self.match_token(vec![
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
            TokenType::LeftParen,
        ]) {
            let previous = self.previous();

            match previous.token_type {
                TokenType::True => {
                    return Ok(lexpr(Literal::Boolean(true)));
                }
                TokenType::False => {
                    return Ok(lexpr(Literal::Boolean(false)));
                }
                TokenType::Nil => {
                    return Ok(lexpr(Literal::Nil));
                }
                TokenType::String | TokenType::Number => {
                    let literal = previous.literal.clone();
                    return Ok(lexpr(literal.unwrap()));
                }
                TokenType::LeftParen => {
                    let expr = self.expression()?;

                    // Panics if unwraps on Err
                    self.consume(TokenType::RightParen, "Expect ')' after expression.")
                        .unwrap();

                    return Ok(gexpr(expr));
                }
                _ => {}
            }
        }

        Err(ParseError {
            token: self.peek().clone(),
            message: "Expected expression.".to_string(),
        })
    }

    fn match_token(&self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            let current = self._current.get();
            self._current.set(current + 1);
        }
        self.previous()
    }

    fn consume(&self, token_type: TokenType, error: &str) -> Result<&Token, ParseError> {
        if self.match_token(vec![token_type]) {
            return Ok(self.advance());
        }

        Err(ParseError {
            token: self.peek().clone(),
            message: error.to_string(),
        })
    }

    fn synchronize(&self) {
        use crate::ast::tokentype::TokenType::*;
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self._current.get()).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self._current.get() - 1).unwrap()
    }

    fn error(&self, token: &Token, message: &str) {
        match self._reporter {
            Some(reporter) => reporter.error(token, message),

            // Reporter does not exist, print to stderr
            None => eprintln!("[Error]: {}", message),
        }
    }
}
