use std::cell::Cell;

use crate::ast::{token::Token, tokentype::TokenType};

pub struct ErrorReporter {
    has_error: Cell<bool>,
}

impl ErrorReporter {
    pub fn new() -> ErrorReporter {
        ErrorReporter {
            has_error: Cell::new(false),
        }
    }

    pub fn has_error(&self) -> bool {
        self.has_error.get()
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

    pub fn report(&self, line: usize, place: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, place, message);
        self.has_error.set(true);
    }
}
