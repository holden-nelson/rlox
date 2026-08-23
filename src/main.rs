mod lox;
mod tokens;
mod scanner;

use std::{env, process::ExitCode};

use crate::lox::Lox;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let lox = Lox::new();

    let result = match args.as_slice() {
        [] => lox.run_prompt(),
        [script] => lox.run_file(script),
        _ => {
            eprintln!("Usage: rlox [script]");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
