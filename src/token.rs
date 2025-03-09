//! Defines token types used in KdnLang after lexical analysis.

/// Tokens in the KdnLang language produced by the lexer.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    /// Identifier (variable/function name) - String value
    Identifier(String),
    /// Numeric literal - Integer value
    Number(i32),
    /// Addition operator '+'
    Plus,
    /// Multiplication operator '*'
    Asterisk,
    /// Left parenthesis '('
    LeftParen,
    /// Right parenthesis ')'
    RightParen,
    /// Subtraction operator '-'
    Minus,
    /// Division operator '/'
    Slash,
    /// Unrecognized character
    Unknown(char),
}
