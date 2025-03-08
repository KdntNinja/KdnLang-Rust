#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,

    // Literals
    Identifier(String),
    Number(i64),

    // Special tokens
    Whitespace(usize), // Count of spaces
    Newline,

    // Operators
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Equals,      // =
    GreaterThan, // >
    LessThan,    // <
    And,         // &&
    Or,          // ||
    Not,         // !

    // Delimiters
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    LeftCurlyBracket,  // {
    RightCurlyBracket, // }
    Semicolon,         // ;
    Colon,             // :
    Comma,             // ,
}
