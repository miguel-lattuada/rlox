use super::{tokentype::Literal, tokentype::TokenType};
use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<Literal>,
        line: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] ({:?}) {} {:?}",
            self.line, self.token_type, self.lexeme, self.literal
        )
    }
}
