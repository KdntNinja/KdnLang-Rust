use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic)]
pub enum LexerError {
    #[error("Unexpected character '{character}'")]
    #[diagnostic(code(lexer::unexpected_character))]
    UnexpectedCharacter {
        character: char,
        #[source_code]
        src: String,
        #[label("unexpected character")]
        span: SourceSpan,
    },

    #[error("Invalid number '{lexeme}'")]
    #[diagnostic(code(lexer::invalid_number))]
    InvalidNumber {
        lexeme: String,
        #[source_code]
        src: String,
        #[label("invalid number")]
        span: SourceSpan,
    },
}
