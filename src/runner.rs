use crate::error::ErrorReporter;
use crate::scanner::Scanner;
use std::{fs, io, io::Write, process};

pub struct Runner {
    error_reporter: ErrorReporter,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            error_reporter: ErrorReporter::new(),
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(&source);
        scanner.set_error_reporter(&self.error_reporter);

        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn run_file(&self, file: &String) {
        let file_bytes = fs::read(file).unwrap();
        let file_str = String::from_utf8(file_bytes).unwrap();

        self.run(file_str);

        if self.error_reporter.has_error() {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();

            assert_eq!(line.pop(), Some('\n'));

            if line.is_empty() {
                break;
            }

            self.run(line);
            self.error_reporter.reset();
        }
    }
}
