mod interpreter;
mod lexer;
mod parser;
mod token;

use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::token::Token;
use miette::{Report, Result, miette};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let filename: String = env::args()
        .nth(1)
        .ok_or_else(|| miette!("No filename provided"))?;

    let text: String = fs::read_to_string(&filename)
        .map_err(|e| Report::msg(format!("Failed to read file '{}': {}", filename, e)))?;
    let tokens: Vec<Token> = tokenize(&text)?;
    for i in &tokens {
        println!("{:?}", i);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);

    Ok(())
}
