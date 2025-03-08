use crate::lexer::core::lexer::Lexer;
use crate::lexer::error::LexerError;
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;
use miette::Result;

pub fn read_number(lexer: &mut Lexer) -> Result<Token, LexerError> {
    let start = lexer.position;
    while lexer.ch.map_or(false, |c| c.is_ascii_digit()) {
        lexer.advance();
    }
    let lexeme: String = lexer.input[start..lexer.position].iter().collect();
    let kind = match lexeme.parse::<i64>() {
        Ok(num) => TokenKind::Number(num),
        Err(_) => {
            return Err(LexerError::InvalidNumber {
                lexeme,
                src: lexer.source.clone(),
                span: (start, lexer.position - start).into(),
            });
        }
    };
    Ok(Token::new(kind, start, lexer.position))
}

pub fn read_identifier(lexer: &mut Lexer) -> Result<Token, LexerError> {
    let start = lexer.position;
    while lexer
        .ch
        .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
    {
        lexer.advance();
    }
    let lexeme: String = lexer.input[start..lexer.position].iter().collect();
    Ok(Token::new(
        TokenKind::Identifier(lexeme),
        start,
        lexer.position,
    ))
}

pub fn read_string(lexer: &mut Lexer) -> Result<Token, LexerError> {
    // Skip the opening quote
    let start = lexer.position;
    lexer.advance();

    let mut string = String::new();

    // Read until closing quote or EOF
    while let Some(ch) = lexer.ch {
        if ch == '"' {
            // Found the closing quote
            lexer.advance(); // Skip closing quote
            return Ok(Token::new(TokenKind::String(string), start, lexer.position));
        } else if ch == '\n' || ch == '\r' {
            // Strings can't contain newlines
            return Err(LexerError::UnterminatedString {
                src: lexer.source.clone(),
                span: (start, lexer.position - start).into(),
            });
        } else if ch == '\\' {
            // Handle escape sequences
            lexer.advance(); // Skip the backslash
            match lexer.ch {
                Some('n') => string.push('\n'),
                Some('r') => string.push('\r'),
                Some('t') => string.push('\t'),
                Some('\\') => string.push('\\'),
                Some('"') => string.push('"'),
                Some(c) => {
                    return Err(LexerError::InvalidEscapeSequence {
                        character: c,
                        src: lexer.source.clone(),
                        span: (lexer.position - 1, 2).into(),
                    });
                }
                None => {
                    return Err(LexerError::UnterminatedString {
                        src: lexer.source.clone(),
                        span: (start, lexer.position - start).into(),
                    });
                }
            }
            lexer.advance();
        } else {
            // Add character to string and continue
            string.push(ch);
            lexer.advance();
        }
    }

    // If we get here, we reached EOF without finding a closing quote
    Err(LexerError::UnterminatedString {
        src: lexer.source.clone(),
        span: (start, lexer.position - start).into(),
    })
}
