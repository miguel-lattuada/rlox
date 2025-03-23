use super::{
    keywords::get_keyword_token_type,
    token::Token,
    tokentype::{Literal, TokenType},
};
use crate::error::ErrorReporter;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,

    _reporter: Option<&'a ErrorReporter>,
    _source: Vec<char>,
    _start: usize,
    _current: usize,
    _line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),

            _source: source.chars().collect::<Vec<char>>(),
            _reporter: None,
            _start: 0,
            _current: 0,
            _line: 1,
        }
    }

    pub fn set_error_reporter(&mut self, reporter: &'a ErrorReporter) {
        self._reporter = Some(reporter);
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self._start = self._current;
            self.scan_token();
        }

        let eof_token = Token::new(TokenType::Eof, "", None, 0);
        self.tokens.push(eof_token);

        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            // Single char tokens
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('{') => self.add_token(TokenType::LeftBrace),
            Some('}') => self.add_token(TokenType::RightBrace),
            Some(',') => self.add_token(TokenType::Comma),
            Some('.') => self.add_token(TokenType::Dot),
            Some('-') => self.add_token(TokenType::Minus),
            Some('+') => self.add_token(TokenType::Plus),
            Some(';') => self.add_token(TokenType::Semicolon),
            Some('*') => self.add_token(TokenType::Star),

            // Single or double char tokens
            Some('!') => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            Some('=') => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            Some('<') => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            Some('>') => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }

            // Meaningless characters
            Some(' ') => (),
            Some('\r') => (),
            Some('\t') => (),
            Some('\n') => self._line += 1,

            // Multi char tokens
            Some('/') => {
                if self.match_char('/') {
                    self.ignore_until_newline();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            Some('"') => self.scan_string('"'),
            Some('\'') => self.scan_string('\''),

            None => (),
            _ => {
                let char = char.unwrap();

                if char.is_numeric() {
                    self.scan_number();
                    return;
                }

                if char.is_alphabetic() || *char == '_' {
                    self.scan_identifier();
                    return;
                }

                let message = format!("Unexpected character: '{}'", char);
                self.report(self._line, "", &message)
            }
        }
    }

    fn scan_string(&mut self, delimmeter: char) {
        while self.peek() != delimmeter && !self.is_at_end() {
            if self.peek() == '\n' {
                self._line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report(self._line, "", "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self._start + 1..self._current - 1];
        self.add_token_literal(TokenType::String, Some(Literal::String(value.to_string())));
    }

    fn scan_number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // Consume the "."
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.add_token_literal(
            TokenType::Number,
            Some(Literal::Number(
                self.source[self._start..self._current]
                    .parse::<f64>()
                    .unwrap(),
            )),
        );
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source[self._start..self._current];
        let token_type = get_keyword_token_type(text);

        let token = match token_type {
            Some(token_type) => token_type,
            None => TokenType::Identifier,
        };

        self.add_token(token);
    }

    fn advance(&mut self) -> Option<&char> {
        self._current += 1;
        self._source.get(self._current - 1)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let token = self._source.get(self._current);

        match token {
            Some(token) if *token == expected => {
                self._current += 1;
                return true;
            }
            _ => false,
        }
    }

    fn ignore_until_newline(&mut self) {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self._source[self._current]
    }

    fn peek_next(&self) -> char {
        let next = self._current + 1;
        if next >= self.source.len() {
            return '\0';
        }
        self._source[next]
    }

    fn is_at_end(&self) -> bool {
        self._current >= self.source.len()
    }

    fn add_token(&mut self, token: TokenType) {
        let token = Token::new(token, "", None, self._line);
        self.tokens.push(token);
    }

    fn add_token_literal(&mut self, token: TokenType, literal: Option<Literal>) {
        let lexeme = &self.source[self._start..self._current];
        let token = Token::new(token, lexeme, literal, self._line);
        self.tokens.push(token);
    }

    fn report(&self, line: usize, place: &str, message: &str) {
        match self._reporter {
            Some(reporter) => reporter.report(line, place, message),

            // Reporter does not exist, print to stderr
            None => eprintln!("[line {}] Error{}: {}", line, place, message),
        }
    }
}
