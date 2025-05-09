use std::cell::Cell;

use crate::ast::expr::{aexpr, cexpr, lgexpr, vexpr};
use crate::ast::stmt::{fstmt, ifstmt, vdstmt, wstmt};
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
        Self {
            _reporter: None,
            _current: Cell::new(0),
            tokens,
        }
    }

    pub fn set_error_reporter(&mut self, reporter: &'a ErrorReporter) {
        self._reporter = Some(reporter);
    }

    pub fn parse(&self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt);
                }
                Err(e) => {
                    self.error(&e.token, e.message.as_str());
                    self.synchronize();
                }
            }
        }

        statements
    }

    /**
    * Parse grammar rule: declaration    → statement
                                           | varDecl ;
    */
    fn declaration(&self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenType::Fun]) {
            return self.fun_decl_stmt("function");
        }

        if self.match_token(vec![TokenType::Var]) {
            return self.var_decl_stmt();
        }

        self.statement()
    }

    /**
    * Parse grammar rule: statement      → exprStmt
                                           | printStmt ;
    */
    fn statement(&self) -> Result<Stmt, ParseError> {
        if self.match_token(vec![TokenType::For]) {
            return self.for_stmt();
        }

        if self.match_token(vec![TokenType::If]) {
            return self.if_stmt();
        }

        if self.match_token(vec![TokenType::Print]) {
            return self.print_stmt();
        }

        if self.match_token(vec![TokenType::While]) {
            return self.while_stmt();
        }

        if self.match_token(vec![TokenType::Return]) {
            return self.return_stmt();
        }

        if self.match_token(vec![TokenType::LeftBrace]) {
            let stmts = self.block()?;
            return Ok(Stmt::Block(stmts));
        }

        self.expression_stmt()
    }

    /**
     * Parse grammar rule: block          → "{" declaration* "}" ;
     */
    fn block(&self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(stmts)
    }

    /**
    * Parse grammar rule: ifStmt         → "if" "(" expression ")" statement
                                            ( "else" statement )? ;
    */
    fn if_stmt(&self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;

        let mut else_branch: Option<Stmt> = None;

        if self.match_token(vec![TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(ifstmt(condition, then_branch, else_branch))
    }

    /**
    * Parse grammar rule: forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                                            expression? ";"
                                            expression? ")" statement ;
    */
    fn for_stmt(&self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Stmt> = None;
        if self.match_token(vec![TokenType::Semicolon]) {
            initializer = None;
        } else if self.match_token(vec![TokenType::Var]) {
            initializer = Some(self.var_decl_stmt()?);
        } else {
            initializer = Some(self.expression_stmt()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(i) = increment {
            body = Stmt::Block(vec![body, estmt(i)]);
        }

        if let Some(c) = condition {
            body = wstmt(c, body);
        } else {
            body = wstmt(lexpr(Literal::Boolean(true)), body);
        }

        if let Some(init) = initializer {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    /**
     * Parse grammar rule: whileStmt      → "while" "(" expression ")" statement ;
     */
    fn while_stmt(&self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;

        let body = self.statement()?;

        Ok(wstmt(condition, body))
    }

    /**
     *Parse grammar rule: returnStmt     → "return" expression? ";";
     */
    fn return_stmt(&self) -> Result<Stmt, ParseError> {
        let token = self.previous();
        let mut return_expr = lexpr(Literal::Nil);

        if !self.match_token(vec![TokenType::Semicolon]) {
            return_expr = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

        Ok(Stmt::Return(token.clone(), return_expr))
    }

    /** Parse gramma rule: funDecl        → "fun" function ;
     *                     function       → IDENTIFIER "(" parameters? ")" block ;
     */
    fn fun_decl_stmt(&self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;

        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;

        let mut parameters = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() > 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters");
                }

                parameters.push(
                    self.consume(TokenType::Identifier, "Expect parameter name.")?
                        .clone(),
                );

                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' to start {} body.", kind).as_str(),
        )?;

        let body = self.block()?;

        Ok(fstmt(name.clone(), parameters, Stmt::Block(body)))
    }

    /**
     * Parse grammar rule: varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
     */
    fn var_decl_stmt(&self) -> Result<Stmt, ParseError> {
        let token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut expr = None;

        if self.match_token(vec![TokenType::Equal]) {
            expr = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(vdstmt(token.clone(), expr))
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
        self.assignment()
    }

    /**
    * Parse grammar rule: assignment     → IDENTIFIER "=" assignment
                                            | equality ;
    */
    fn assignment(&self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_token(vec![TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            return match expr {
                Expr::VariableExpr(ref token) => Ok(aexpr(token.clone(), value)),
                _ => Err(ParseError {
                    token: equals.clone(),
                    message: "Invalid assignment target.".to_string(),
                }),
            };
        }

        Ok(expr)
    }

    /**
     * Parse grammar rule: logic_or       → logic_and ( "or" logic_and )* ;
     */
    fn or(&self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = lgexpr(expr, operator.clone(), right);
        }
        Ok(expr)
    }

    /**
     * Parse grammar rule: logic_and      → equality ( "and" equality )* ;
     */
    fn and(&self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = lgexpr(expr, operator.clone(), right);
        }
        Ok(expr)
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
        self.call()
    }

    /**
     * Parse grammar rule: call           → primary ( "(" arguments? ")" )* ;z
     */
    fn call(&self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /**
     * Parse grammar rule: arguments      → expression ( "," expression )* ;
     */
    fn finish_call(&self, callee: Expr) -> Result<Expr, ParseError> {
        let mut args = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() > 255 {
                    return Err(ParseError {
                        token: self.peek().clone(),
                        message: "Can't have more than 255 arguments.".to_string(),
                    });
                }

                args.push(self.expression()?);

                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(cexpr(callee, paren.clone(), args))
    }

    /**
     * Parse grammer rule: primary        → NUMBER | STRING | "true" | "false" | "nil"
     *                                      | "(" expression ")"
     *                                      | IDENTIFIER ;
     */
    fn primary(&self) -> Result<Expr, ParseError> {
        if self.match_token(vec![
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
            TokenType::LeftParen,
            TokenType::Identifier,
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
                TokenType::Identifier => return Ok(vexpr(previous.clone())),
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
        if self.check(token_type) {
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
