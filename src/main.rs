mod lexer;

use crate::lexer::lexer::Lexer;
use crate::lexer::token_kind::TokenKind;
use miette::{Report, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| Report::msg("No filename provided"))?;
    let text = fs::read_to_string(&filename).unwrap();

    let mut lexer = Lexer::new(&text);

    loop {
        match lexer.next_token() {
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

    Ok(())
}
