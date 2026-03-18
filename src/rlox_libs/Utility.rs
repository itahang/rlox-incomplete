#![allow(non_snake_case)]

use anyhow::{Context, Ok, Result};
use once_cell::sync::Lazy;
use std::io::{BufRead, Write, stdout};
use std::sync::Mutex;

use crate::rlox_libs::ErrorReporting::ErrorStatus;
use crate::rlox_libs::Scanner::Scanner;

pub static ERROR_STATUS: Lazy<Mutex<ErrorStatus>> = Lazy::new(|| Mutex::new(ErrorStatus::new()));

pub fn runScript(path: &String) -> Result<()> {
    let s =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))?;

    run(s);

    if ERROR_STATUS.lock().unwrap().status() {
        return Ok(());
    }

    return Ok(());
}
pub fn runREPL() -> Result<()> {
    let mut stdin = std::io::stdin().lock();

    loop {
        print!("> ");
        stdout().flush()?;

        let mut line = String::new();

        stdin.read_line(&mut line)?;
        let line = line.trim();

        if line == "exit" {
            break;
        }
        run(line.to_string());

        ERROR_STATUS.lock().unwrap().reset();
    }

    Ok(())
}

pub fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scanTokens();

    for token in tokens {
        println!("{}", token);
    }
}
