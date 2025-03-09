//! Lexical analysis (tokenization) for KdnLang using Logos.

use crate::token::Token;
use logos::Logos;
use miette::miette;

/// Logos-specific token types for internal lexer use.
#[derive(Logos, Debug, PartialEq)]
pub enum LogosToken {
    /// '+'
    #[token("+")]
    Plus,
    
    /// '-'
    #[token("-")]
    Minus,
    
    /// The multiplication operator '*'
    #[token("*")]
    Asterisk,
    
    /// The division operator '/'
    #[token("/")]
    Slash,
    
    /// The left parenthesis '('
    #[token("(")]
    LeftParen,
    
    /// The right parenthesis ')'
    #[token(")")]
    RightParen,
    
    /// Integer literal - parses digits to i32
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i32>().map_err(|_| ()))]
    Number(i32),
    
    /// Identifier - letter followed by alphanumerics
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*", |lex| String::from(lex.slice()))]
    Identifier(String),
    
    /// Skipped whitespace
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Converts source code string into tokens.
/// Returns Result with token vector or error diagnostic.
pub fn tokenize(source_code: &str) -> miette::Result<Vec<Token>> {
    // Create a new Logos lexer with the source code
    let mut lexer = LogosToken::lexer(source_code);

    // Convert the Logos tokens into our Token enum
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(lexeme) = lexer.next() {
        match lexeme {
            Ok(LogosToken::Plus) => tokens.push(Token::Plus),
            Ok(LogosToken::Minus) => tokens.push(Token::Minus),
            Ok(LogosToken::Asterisk) => tokens.push(Token::Asterisk),
            Ok(LogosToken::Slash) => tokens.push(Token::Slash),
            Ok(LogosToken::LeftParen) => tokens.push(Token::LeftParen),
            Ok(LogosToken::RightParen) => tokens.push(Token::RightParen),
            Ok(LogosToken::Number(n)) => tokens.push(Token::Number(n)),
            Ok(LogosToken::Identifier(id)) => tokens.push(Token::Identifier(id)),
            Ok(LogosToken::Error) => {
                tokens.push(Token::Unknown(lexer.slice().chars().next().unwrap()))
            }
            Err(_) => {
                return Err(miette!("Lexer error at position {}", lexer.span().start));
            }
        }
    }

    Ok(tokens)
}
