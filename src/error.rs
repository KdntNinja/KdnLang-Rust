use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum LexerError {
    #[error("Unexpected character: {character}")]
    #[diagnostic(
        code = "lexer.unexpected_character",
        help = "Check if this is a valid token"
    )]
    UnexpectedCharacter {
        character: char,
        #[source_code]
        src: String,
        #[label("Unexpected character here")]
        span: SourceSpan,
    },

    #[error("Invalid number: {lexeme}")]
    #[diagnostic(
        code = "lexer.invalid_number",
        help = "Check for invalid digits or formatting"
    )]
    InvalidNumber {
        lexeme: String,
        #[source_code]
        src: String,
        #[label("Invalid number")]
        span: SourceSpan,
    },
}
