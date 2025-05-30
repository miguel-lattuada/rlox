use std::{env, process};

mod ast;
mod error;
mod interpreter;
mod parser;
mod runner;
mod scanner;

use runner::Runner;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        process::exit(64);
    }

    let mut runner = Runner::new();

    let script_path = args.get(1);

    if let Some(script) = script_path {
        runner.run_file(script);
    } else {
        runner.run_prompt();
    }
}
