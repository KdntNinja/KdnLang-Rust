use crate::lexer::core::lexer::Lexer;
use crate::lexer::error::LexerError;
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;
use miette::Result;

pub fn read_operator(lexer: &mut Lexer) -> Result<Token, LexerError> {
    match lexer.ch {
        Some('+') => Ok(lexer.token(TokenKind::Plus)),
        Some('-') => Ok(lexer.token(TokenKind::Minus)),
        Some('*') => Ok(lexer.token(TokenKind::Asterisk)),
        Some('/') => Ok(lexer.token(TokenKind::Slash)),
        Some('=') => {
            lexer.advance();
            // Check if the next character is also '=' for double equals
            if lexer.ch == Some('=') {
                lexer.advance();
                Ok(Token::new(
                    TokenKind::DoubleEquals,
                    lexer.position - 2,
                    lexer.position,
                ))
            } else {
                Ok(Token::new(
                    TokenKind::Equals,
                    lexer.position - 1,
                    lexer.position,
                ))
            }
        }
        Some('>') => {
            lexer.advance();
            // Check if the next character is '=' for '>='
            if lexer.ch == Some('=') {
                lexer.advance();
                Ok(Token::new(
                    TokenKind::GreaterThanEquals,
                    lexer.position - 2,
                    lexer.position,
                ))
            } else {
                Ok(Token::new(
                    TokenKind::GreaterThan,
                    lexer.position - 1,
                    lexer.position,
                ))
            }
        }
        Some('<') => {
            lexer.advance();
            // Check if the next character is '=' for '<='
            if lexer.ch == Some('=') {
                lexer.advance();
                Ok(Token::new(
                    TokenKind::LessThanEquals,
                    lexer.position - 2,
                    lexer.position,
                ))
            } else {
                Ok(Token::new(
                    TokenKind::LessThan,
                    lexer.position - 1,
                    lexer.position,
                ))
            }
        }
        Some('(') => Ok(lexer.token(TokenKind::LeftParen)),
        Some(')') => Ok(lexer.token(TokenKind::RightParen)),
        Some('[') => Ok(lexer.token(TokenKind::LeftBracket)),
        Some(']') => Ok(lexer.token(TokenKind::RightBracket)),
        Some('{') => Ok(lexer.token(TokenKind::LeftCurlyBracket)),
        Some('}') => Ok(lexer.token(TokenKind::RightCurlyBracket)),
        Some(';') => Ok(lexer.token(TokenKind::Semicolon)),
        Some(':') => Ok(lexer.token(TokenKind::Colon)),
        Some(',') => Ok(lexer.token(TokenKind::Comma)),
        Some('&') => {
            lexer.advance();
            if lexer.ch == Some('&') {
                lexer.advance();
                Ok(lexer.token(TokenKind::And))
            } else {
                Err(LexerError::UnexpectedCharacter {
                    character: '&',
                    src: lexer.source.clone(),
                    span: (lexer.position, 1).into(),
                })
            }
        }
        Some('|') => {
            lexer.advance();
            if lexer.ch == Some('|') {
                lexer.advance();
                Ok(lexer.token(TokenKind::Or))
            } else {
                Err(LexerError::UnexpectedCharacter {
                    character: '|',
                    src: lexer.source.clone(),
                    span: (lexer.position, 1).into(),
                })
            }
        }
        Some('!') => Ok(lexer.token(TokenKind::Not)),
        Some(c) => Err(LexerError::UnexpectedCharacter {
            character: c,
            src: lexer.source.clone(),
            span: (lexer.position, 1).into(),
        }),
        None => unreachable!("EOF should be handled in the main lexer"),
    }
}
