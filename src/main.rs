use anyhow::Result;
use std::env::args;

pub mod rlox_libs;
use rlox_libs::Utility::*;

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
