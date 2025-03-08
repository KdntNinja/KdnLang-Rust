use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum LexerError {
    #[error("Unexpected character: {character}")]
    #[diagnostic(code(lexer.unexpected_character), help("Check if this is a valid token"))]
    UnexpectedCharacter {
        character: char,
        #[source_code]
        src: String,
        #[label("Here")]
        span: SourceSpan,
    },

    #[error("Invalid number: {lexeme}")]
    #[diagnostic(code(lexer.invalid_number))]
    InvalidNumber {
        lexeme: String,
        #[source_code]
        src: String,
        #[label("This is not a valid number")]
        span: SourceSpan,
    },
}
