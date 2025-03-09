//! Main entry point for the KdnLang interpreter.
//! Pipeline: tokenization → parsing → interpretation

mod interpreter;
mod lexer;
mod parser;
mod token;

use crate::interpreter::Interpreter;
use crate::lexer::tokenize;
use crate::token::Token;
use miette::{Report, Result, miette};
use std::env;
use std::fs;

/// Drives the KdnLang interpreter pipeline:
/// 1. Reads source file from command-line argument
/// 2. Tokenizes source code
/// 3. Parses tokens into AST
/// 4. Interprets AST for result
fn main() -> Result<()> {
    // Get the filename from the command line arguments.
    let filename: String = env::args()
        .nth(1)
        .ok_or_else(|| miette!("No filename provided"))?;

    // Read the contents of the file.
    let text: String = fs::read_to_string(&filename)
        .map_err(|e| Report::msg(format!("Failed to read file '{}': {}", filename, e)))?;

    // Tokenize the source code using the Logos lexer.
    let tokens: Vec<Token> = tokenize(&text)?;
    for i in &tokens {
        println!("{:?}", i);
    }

    // Create a new parser with the tokens.
    let mut parser = crate::parser::KdnLangParser::new(tokens);
    // Parse the tokens into an abstract syntax tree (AST).
    let ast = parser.parse();

    println!("{:#?}", ast);

    // If the AST is valid, evaluate it using the interpreter.
    if let Some(ast) = ast {
        let interpreter = Interpreter;
        let result = interpreter.visit(&ast);
        println!("Result: {}", result);
    }

    Ok(())
}
