use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic, Clone)]
pub enum InterpreterError {
    #[error("Undefined variable '{name}'")]
    #[diagnostic(code(interpreter::undefined_variable))]
    UndefinedVariable {
        name: String,
        #[source_code]
        src: String,
        #[label("variable not defined")]
        span: SourceSpan,
    },

    #[error("Type error: {message}")]
    #[diagnostic(code(interpreter::type_error))]
    TypeError {
        message: String,
        #[source_code]
        src: String,
        #[label("type error")]
        span: SourceSpan,
    },

    #[error("Runtime error: {message}")]
    #[diagnostic(code(interpreter::runtime_error))]
    RuntimeError {
        message: String,
        #[source_code]
        src: String,
        #[label("runtime error")]
        span: SourceSpan,
    },
}
