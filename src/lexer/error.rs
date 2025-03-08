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

    #[error("Unterminated string literal")]
    #[diagnostic(code(lexer::unterminated_string))]
    UnterminatedString {
        #[source_code]
        src: String,
        #[label("unterminated string")]
        span: SourceSpan,
    },

    #[error("Indentation error")]
    #[diagnostic(code(lexer::indentation_error))]
    IndentationError {
        #[source_code]
        src: String,
        #[label("inconsistent indentation")]
        span: SourceSpan,
    },

    #[error("Invalid escape sequence '\\{character}'")]
    #[diagnostic(code(lexer::invalid_escape_sequence))]
    InvalidEscapeSequence {
        character: char,
        #[source_code]
        src: String,
        #[label("invalid escape sequence")]
        span: SourceSpan,
    },
}
