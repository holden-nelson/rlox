use std::{fs, io::{self, BufRead, Write}};
use thiserror::Error;

use crate::scanner::Scanner;

pub struct Lox { }

impl Lox {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run_file(&self, path: &str) -> Result<(), LoxError> {
        let source = fs::read_to_string(path)?;
        Self::run(&source)?;
        Ok(())
    }

    pub fn run_prompt(&self) -> Result<(), LoxError> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        loop {
            write!(stdout, "> ")?;
            stdout.flush()?;

            let Some(line) = lines.next() else {
                break;
            };

            let line = line?;
            Self::run(&line)?;
        }

        Ok(())
    }

    fn run(source: &str) -> Result<(), LoxError> {
        let tokens = Scanner::new(source).scan()?;

        for token in &tokens {
            println!("{token}");
        }

        Ok(())
    }


}

#[derive(Debug, Error)]
#[error("[line {line}] Error {at}: {message}")]
pub struct CompileError {
    pub line: usize,
    pub at: String,
    pub message: String
}

#[derive(Debug, Error)]
#[error("[line {line}] Error {at}: {message}")]
pub struct RunTimeError {
    pub line: usize,
    pub at: String,
    pub message: String
}

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error(transparent)]
    Compile(CompileError),

    #[error(transparent)]
    RunTime(RunTimeError)
}