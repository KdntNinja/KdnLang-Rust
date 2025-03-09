#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Asterisk,
    LeftParen,
    RightParen,
    Minus,
    Slash,
    Unknown(char),
}
