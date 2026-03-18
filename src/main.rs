#![allow(non_snake_case)]
use anyhow::{Context, Result};
use std::{
    env::args,
    io::{BufRead, Write, stdout},
};

use crate::rlox_libs::Scanner::Scanner;
pub mod rlox_libs;

fn runScript(path: &String) -> Result<()> {
    let s =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))?;

    println!("{s}");

    return Ok(());
}
fn runREPL() -> Result<()> {
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
        run( line.to_string());
    }

    Ok(())
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scanTokens().iter();

    for token in tokens {
        println!("{}", token);
    }
}

fn main() -> Result<()> {
    let arg: Vec<String> = args().collect();

    if arg.len() > 2 {
        println!("Usage: rlox [script]");
        return Ok(());
    } else if arg.len() == 2 {
        runScript(&arg[1].to_string())?;
    } else {
        runREPL()?;
    }

    Ok(())
}
