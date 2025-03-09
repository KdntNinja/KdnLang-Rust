mod lexer;
mod parser;
mod interpreter;
mod tokens;

use miette::{Report, Result, miette};
use std::env;
use std::fs;
use crate::lexer::tokenize;
use crate::tokens::Tokens;

fn main() -> Result<()> {
    let filename: String = env::args()
        .nth(1)
        .ok_or_else(|| miette!("No filename provided"))?;

    let text: String = fs::read_to_string(&filename)
        .map_err(|e| Report::msg(format!("Failed to read file '{}': {}", filename, e)))?;

    let tokens: Vec<Tokens> = tokenize(&text)?;
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
