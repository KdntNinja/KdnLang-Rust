use crate::error::LexerError;
use crate::token::Token;
use crate::token_kind::TokenKind;
use miette::Result;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    source: String,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
            source: input.to_string(),
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.ch = self.input.get(self.read_position).copied();
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        loop {
            let token = match self.ch {
                Some(ch) if ch.is_whitespace() => {
                    self.advance();
                    continue;
                }
                Some('+') => Ok(self.token(TokenKind::Plus)),
                Some('-') => Ok(self.token(TokenKind::Minus)),
                Some('*') => Ok(self.token(TokenKind::Asterisk)),
                Some('/') => Ok(self.token(TokenKind::Slash)),
                Some('=') => Ok(self.token(TokenKind::Equals)),
                Some('(') => Ok(self.token(TokenKind::LeftParen)),
                Some(')') => Ok(self.token(TokenKind::RightParen)),
                Some('[') => Ok(self.token(TokenKind::LeftBracket)),
                Some(']') => Ok(self.token(TokenKind::RightBracket)),
                Some('{') => Ok(self.token(TokenKind::LeftCurlyBracket)),
                Some('}') => Ok(self.token(TokenKind::RightCurlyBracket)),
                Some(';') => Ok(self.token(TokenKind::Semicolon)),
                Some(':') => Ok(self.token(TokenKind::Colon)),
                Some('0'..='9') => self.read_number(),
                Some('a'..='z') | Some('A'..='Z') | Some('_') => self.read_identifier(),
                Some(c) => Err(LexerError::UnexpectedCharacter {
                    character: c,
                    src: self.source.clone(),
                    span: (self.position, 1).into(),
                }),
                None => Ok(self.token(TokenKind::Eof)),
            };

            return token;
        }
    }

    fn token(&mut self, kind: TokenKind) -> Token {
        let tok = Token::new(kind, self.position, self.read_position);
        self.advance();
        tok
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let start = self.position;
        while self.ch.map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }
        let lexeme: String = self.input[start..self.position].iter().collect();
        let kind = match lexeme.parse::<i64>() {
            Ok(num) => TokenKind::Number(num),
            Err(_) => {
                return Err(LexerError::InvalidNumber {
                    lexeme,
                    src: self.source.clone(),
                    span: (start, self.position - start).into(),
                });
            }
        };
        Ok(Token::new(kind, start, self.position))
    }

    fn read_identifier(&mut self) -> Result<Token, LexerError> {
        let start = self.position;
        while self
            .ch
            .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }
        let lexeme: String = self.input[start..self.position].iter().collect();
        Ok(Token::new(
            TokenKind::Identifier(lexeme),
            start,
            self.position,
        ))
    }
}
