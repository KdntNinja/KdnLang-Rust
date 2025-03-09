use std::iter::Peekable;
use std::str::Chars;
use crate::tokens::Tokens;

pub fn tokenize(source_code: &str) -> miette::Result<Vec<Tokens>> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut chars: Peekable<Chars> = source_code.chars().peekable();

    while let Some(ch) = chars.next() {
        let token: Tokens = match ch {
            '+' => Tokens::Plus,
            '-' => Tokens::Minus,
            '*' => Tokens::Asterisk,
            '/' => Tokens::Slash,
            '(' => Tokens::LeftParen,
            ')' => Tokens::RightParen,
            '0'..='9' => {
                let mut number = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                Tokens::Number(number)
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
                Tokens::Identifier(identifier)
            }
            _ if ch.is_whitespace() => continue,
            _ => Tokens::Unknown(ch),
        };
        tokens.push(token);
    }

    Ok(tokens)
}
