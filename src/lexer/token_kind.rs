#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,

    // Literals
    Identifier(String),
    Number(i64),
    String(String),

    // Special tokens
    #[allow(dead_code)]
    Whitespace(usize), // Count of spaces
    Newline,
    Indent(usize), // Increase in indentation level with size
    Dedent(usize), // Decrease in indentation level with size

    // Operators
    Plus,              // +
    Minus,             // -
    Asterisk,          // *
    Slash,             // /
    Equals,            // =
    DoubleEquals,      // ==
    GreaterThan,       // >
    LessThan,          // <
    LessThanEquals,    // <=
    GreaterThanEquals, // >=
    And,               // &&
    Or,                // ||
    Not,               // !

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
