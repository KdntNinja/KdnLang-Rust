#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,

    // Literals
    Identifier(String),
    Number(i64),
    
    // Operators
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Equals,   // =

    // Delimiters
    LeftParen,         // (
    RightParen,        // )
    LeftBracket,       // [
    RightBracket,      // ]
    LeftCurlyBracket,  // {
    RightCurlyBracket, // }
    Semicolon,         // ;
    Colon,             // :
}
