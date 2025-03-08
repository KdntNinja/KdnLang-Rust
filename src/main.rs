mod lexer;
mod parser;

use crate::lexer::lexer::Lexer;
use crate::lexer::token_kind::TokenKind;
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

    // Debug: Print tokens
    println!("Tokens:");
    let mut token_debug_lexer = Lexer::new(&text);
    loop {
        match token_debug_lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
                if token.kind() == &TokenKind::Eof {
                    break;
                }
            }
            Err(err) => {
                eprintln!("{}", Report::new(err.clone()));
                return Err(err.into());
            }
        }
    }

    // Parse tokens into AST
    let mut parser = Parser::new(&mut lexer, text.clone())?;
    match parser.parse_program() {
        Ok(program) => {
            println!("\nAST:");
            println!("{:#?}", program);
        }
        Err(err) => {
            // Create a report from a clone of the error
            eprintln!("{}", Report::new(err.clone()));
            // Then return the original error
            return Err(err.into());
        }
    }

    Ok(())
}
