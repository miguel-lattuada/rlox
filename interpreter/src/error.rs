use crate::ast::{token::Token, tokentype::TokenType};
use std::cell::Cell;
use std::{error::Error, fmt::Display};

pub struct ErrorReporter {
    has_error: Cell<bool>,
    has_runtime_error: Cell<bool>,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter {
            has_error: Cell::new(false),
            has_runtime_error: Cell::new(false),
        }
    }

    pub fn has_error(&self) -> bool {
        self.has_error.get()
    }

    pub fn has_runtime_error(&self) -> bool {
        self.has_runtime_error.get()
    }

    pub fn reset(&self) {
        self.has_error.set(false);
    }

    pub fn error(&self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!("at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(&self, token: &Token, message: &str) {
        eprintln!(
            "[line {}] Error {}: {}",
            token.line,
            &format!("at '{}'", token.lexeme),
            message
        );
        self.has_runtime_error.set(true);
    }

    pub fn report(&self, line: usize, place: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, place, message);
        self.has_error.set(true);
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for ParseError {}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for RuntimeError {}
