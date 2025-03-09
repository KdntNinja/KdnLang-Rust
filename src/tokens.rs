#[derive(Debug)]
pub enum Tokens {
    Number(String),
    Identifier(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Unknown(char),
}
