use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum ParserError {
    #[error("Unexpected token: expected {expected}, found {found}")]
    #[diagnostic(code(parser::unexpected_token))]
    UnexpectedToken {
        expected: String,
        found: String,
        #[source_code]
        src: String,
        #[label("unexpected token")]
        span: SourceSpan,
    },

    #[error("Missing token: expected {expected}")]
    #[diagnostic(code(parser::missing_token))]
    MissingToken {
        expected: String,
        #[source_code]
        src: String,
        #[label("expected token here")]
        span: SourceSpan,
    },

    #[error("Unexpected end of file")]
    #[diagnostic(code(parser::unexpected_eof))]
    UnexpectedEOF {
        #[source_code]
        src: String,
        #[label("unexpected end of file")]
        span: SourceSpan,
    },
}
