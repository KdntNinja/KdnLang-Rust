mod lexer;
mod parser;
mod interpreter;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use miette::{Report, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| Report::msg("No filename provided"))?;
    let text = fs::read_to_string(&filename).unwrap();

    // Tokenize input
    let mut lexer = Lexer::new(&text);

    // Parse tokens into AST
    let mut parser = Parser::new(&mut lexer, text.clone())?;
    match parser.parse_program() {
        Ok(program) => {
            println!("\nAST:");
            println!("{:#?}", program);
        }
        Err(err) => {
            // Create a report from a clone of the error\
            eprintln!("{}", Report::new(err.clone()));
            // Then return the original error
            return Err(err.into());
        }
    }

    Ok(())
}
