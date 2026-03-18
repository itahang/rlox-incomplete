#![allow(non_snake_case)]
pub mod Utility {
    use anyhow::{Context, Result};
    use std::io::{BufRead, Write, stdout};

    use crate::rlox_libs::Scanner::Scanner;

    pub fn runScript(path: &String) -> Result<()> {
        let s = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path))?;

        println!("{s}");

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
        }

        Ok(())
    }

    pub fn run(source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scanTokens();

        for token in tokens {
            println!("{}", token);
        }
    }
}
