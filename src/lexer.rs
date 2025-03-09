use crate::token::Token;
use logos::Logos;

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
    let mut lexer = Logos::lexer(source_code);

    // Convert the Logos tokens into our Token enum
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(lexeme) = lexer.next() {
        match lexeme {
            Err(LogosToken::Plus) => tokens.push(Token::Plus),
            Err(LogosToken::Minus) => tokens.push(Token::Minus),
            Err(LogosToken::Asterisk) => tokens.push(Token::Asterisk),
            Err(LogosToken::Slash) => tokens.push(Token::Slash),
            Err(LogosToken::LeftParen) => tokens.push(Token::LeftParen),
            Err(LogosToken::RightParen) => tokens.push(Token::RightParen),
            Err(LogosToken::Number(n)) => tokens.push(Token::Number(n)),
            Err(LogosToken::Identifier(id)) => tokens.push(Token::Identifier(id)),
            Err(LogosToken::Error) => tokens.push(Token::Unknown(lexer.slice().chars().next().unwrap())),
        }
    }

    Ok(tokens)
}
