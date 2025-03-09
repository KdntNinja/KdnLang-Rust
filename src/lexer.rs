use logos::Logos;
use crate::token::Token;

// Define the Logos lexer for the language
#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(i32),
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*")]
    Identifier(String),
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

// Function to tokenize the source code using the Logos lexer
pub fn tokenize(source_code: &str) -> miette::Result<Vec<Token>> {
    // Create a new Logos lexer with the source code
    let mut lexer = LogosToken::lexer(source_code);

    // Convert the Logos tokens into our Token enum
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(lexeme) = lexer.next() {
        match lexeme {
            LogosToken::Plus => tokens.push(Token::Plus),
            LogosToken::Minus => tokens.push(Token::Minus),
            LogosToken::Asterisk => tokens.push(Token::Asterisk),
            LogosToken::Slash => tokens.push(Token::Slash),
            LogosToken::LeftParen => tokens.push(Token::LeftParen),
            LogosToken::RightParen => tokens.push(Token::RightParen),
            LogosToken::Number(n) => tokens.push(Token::Number(n)),
            LogosToken::Identifier(id) => tokens.push(Token::Identifier(id)),
            LogosToken::Error => tokens.push(Token::Unknown(lexer.slice().chars().next().unwrap())),
        }
    }

    Ok(tokens)
}
