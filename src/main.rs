mod interpreter;
mod lexer;
mod parser;

use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;
use crate::parser::core::parser::Parser;
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

    // Process the program
    match parser.parse_program() {
        Ok(program) => {
            // Interpret the program
            let mut interpreter = Interpreter::new(text.clone());
            match interpreter.interpret(program) {
                Ok(_) => Ok(()),
                Err(err) => {
                    eprintln!("{}", Report::new(err.clone()));
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            // Create a report from a clone of the error
            eprintln!("{}", Report::new(err.clone()));
            // Then return the original error
            Err(err.into())
        }
    }
}
