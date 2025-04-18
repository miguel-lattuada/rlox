use crate::error::ErrorReporter;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::{fs, io, io::Write, process};

pub struct Runner {
    error_reporter: ErrorReporter,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            error_reporter: ErrorReporter::new(),
        }
    }

    fn run(&self, source: String, interpreter: &mut Interpreter) {
        let mut scanner = Scanner::new(&source);
        scanner.set_error_reporter(&self.error_reporter);
        let tokens = scanner.scan_tokens();

        // Error while scanning
        if self.error_reporter.has_error() {
            return;
        }

        let mut parser = Parser::new(tokens);
        parser.set_error_reporter(&self.error_reporter);

        let statements = parser.parse();

        // Error while parsing
        if self.error_reporter.has_error() {
            return;
        }

        interpreter.interpret(statements);
    }

    pub fn run_file(&self, file: &String) {
        let file_bytes = fs::read(file).unwrap();
        let file_str = String::from_utf8(file_bytes).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.set_error_reporter(&self.error_reporter);

        self.run(file_str, &mut interpreter);

        if self.error_reporter.has_error() {
            process::exit(65);
        }

        if self.error_reporter.has_runtime_error() {
            process::exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        let mut interpreter = Interpreter::new();
        interpreter.set_error_reporter(&self.error_reporter);

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            assert_eq!(line.pop(), Some('\n'));

            if line.is_empty() {
                break;
            }

            self.run(line, &mut interpreter);
            self.error_reporter.reset();
        }
    }
}
