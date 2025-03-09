use crate::token::Token;
use miette::Report;
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(source_code: &str) -> miette::Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars: Peekable<Chars> = source_code.chars().peekable();

    while let Some(ch) = chars.next() {
        let token: Token = match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            '0'..='9' => {
                let mut number = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse().map_err(|e| {
                    Report::msg(format!("Failed to parse number '{}': {}", number, e))
                })?)
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_alphanumeric() || *next == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Identifier(identifier)
            }
            _ if ch.is_whitespace() => continue,
            _ => Token::Unknown(ch),
        };
        tokens.push(token);
    }

    Ok(tokens)
}
