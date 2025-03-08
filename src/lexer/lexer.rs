use crate::lexer::error::LexerError;
use crate::lexer::token::Token;
use crate::lexer::token_kind::TokenKind;
use miette::Result;
use std::collections::VecDeque;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: Option<char>,
    source: String,
    line_start: bool,
    indent_stack: Vec<usize>,
    pending_tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: None,
            source: input.to_string(),
            line_start: true,
            indent_stack: vec![0], // Start with 0 indentation
            pending_tokens: VecDeque::new(),
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
        // First, check if we have pending tokens from previous indentation changes
        if let Some(token) = self.pending_tokens.pop_front() {
            return Ok(token);
        }

        loop {
            // If we're at the start of a line, handle indentation
            if self.line_start && self.ch.is_some() {
                let mut indent_size = 0;
                let indent_start = self.position;

                // Count spaces
                while self.ch == Some(' ') {
                    indent_size += 1;
                    self.advance();
                }

                // If we're at a newline or comment after spaces, just continue
                if self.ch == Some('\n') || self.ch == Some('\r') || self.ch == Some('#') {
                    if self.ch == Some('#') {
                        // Skip comments
                        while self.ch.is_some() && self.ch != Some('\n') && self.ch != Some('\r') {
                            self.advance();
                        }
                    }
                    // Skip the newline
                    if self.ch == Some('\n') || self.ch == Some('\r') {
                        self.advance();
                        if self.ch == Some('\n') && self.input.get(self.position - 2) == Some(&'\r')
                        {
                            self.advance();
                        }
                    }
                    continue;
                }

                // We're no longer at the start of a line
                self.line_start = false;

                // Compare with the current indentation level
                let current_indent = *self.indent_stack.last().unwrap_or(&0);

                if indent_size > current_indent {
                    // Indentation increased
                    self.indent_stack.push(indent_size);
                    return Ok(Token::new(
                        TokenKind::Indent(indent_size),
                        indent_start,
                        self.position,
                    ));
                } else if indent_size < current_indent {
                    // Indentation decreased, may need multiple dedents
                    let mut dedents = Vec::new();
                    while !self.indent_stack.is_empty()
                        && indent_size < *self.indent_stack.last().unwrap()
                    {
                        let prev_indent = self.indent_stack.pop().unwrap();
                        dedents.push(prev_indent);
                    }

                    // If the indentation doesn't match any previous level, it's an error
                    if indent_size != *self.indent_stack.last().unwrap_or(&0) {
                        return Err(LexerError::IndentationError {
                            src: self.source.clone(),
                            span: (indent_start, self.position - indent_start).into(),
                        });
                    }

                    // Queue up dedent tokens
                    for indent in dedents.iter().skip(1) {
                        self.pending_tokens.push_back(Token::new(
                            TokenKind::Dedent(*indent),
                            indent_start,
                            self.position,
                        ));
                    }

                    // Return the first dedent
                    return Ok(Token::new(
                        TokenKind::Dedent(*dedents.first().unwrap_or(&0)),
                        indent_start,
                        self.position,
                    ));
                }
                // If indentation is exactly the same, continue normal processing
            }

            let token = match self.ch {
                Some('\n') | Some('\r') => {
                    let pos = self.position;
                    self.advance();
                    // Handle Windows-style CRLF
                    if self.ch == Some('\n') && self.input.get(self.position - 2) == Some(&'\r') {
                        self.advance();
                    }
                    self.line_start = true;
                    return Ok(Token::new(TokenKind::Newline, pos, self.position));
                }
                Some(ch) if ch.is_whitespace() => {
                    self.advance();
                    continue;
                }
                Some('#') => {
                    // Skip comments
                    while self.ch.is_some() && self.ch != Some('\n') && self.ch != Some('\r') {
                        self.advance();
                    }
                    continue;
                }
                Some('+') => Ok(self.token(TokenKind::Plus)),
                Some('-') => Ok(self.token(TokenKind::Minus)),
                Some('*') => Ok(self.token(TokenKind::Asterisk)),
                Some('/') => Ok(self.token(TokenKind::Slash)),
                Some('=') => {
                    self.advance();
                    // Check if the next character is also '=' for double equals
                    if self.ch == Some('=') {
                        self.advance();
                        Ok(Token::new(
                            TokenKind::DoubleEquals,
                            self.position - 2,
                            self.position,
                        ))
                    } else {
                        Ok(Token::new(
                            TokenKind::Equals,
                            self.position - 1,
                            self.position,
                        ))
                    }
                }
                Some('>') => {
                    self.advance();
                    // Check if the next character is '=' for '>='
                    if self.ch == Some('=') {
                        self.advance();
                        Ok(Token::new(
                            TokenKind::GreaterThanEquals,
                            self.position - 2,
                            self.position,
                        ))
                    } else {
                        Ok(Token::new(
                            TokenKind::GreaterThan,
                            self.position - 1,
                            self.position,
                        ))
                    }
                }
                Some('<') => {
                    self.advance();
                    // Check if the next character is '=' for '<='
                    if self.ch == Some('=') {
                        self.advance();
                        Ok(Token::new(
                            TokenKind::LessThanEquals,
                            self.position - 2,
                            self.position,
                        ))
                    } else {
                        Ok(Token::new(
                            TokenKind::LessThan,
                            self.position - 1,
                            self.position,
                        ))
                    }
                }
                Some('(') => Ok(self.token(TokenKind::LeftParen)),
                Some(')') => Ok(self.token(TokenKind::RightParen)),
                Some('[') => Ok(self.token(TokenKind::LeftBracket)),
                Some(']') => Ok(self.token(TokenKind::RightBracket)),
                Some('{') => Ok(self.token(TokenKind::LeftCurlyBracket)),
                Some('}') => Ok(self.token(TokenKind::RightCurlyBracket)),
                Some(';') => Ok(self.token(TokenKind::Semicolon)),
                Some(':') => Ok(self.token(TokenKind::Colon)),
                Some(',') => Ok(self.token(TokenKind::Comma)),
                Some('&') => {
                    self.advance();
                    if self.ch == Some('&') {
                        self.advance();
                        Ok(self.token(TokenKind::And))
                    } else {
                        Err(LexerError::UnexpectedCharacter {
                            character: '&',
                            src: self.source.clone(),
                            span: (self.position, 1).into(),
                        })
                    }
                }
                Some('|') => {
                    self.advance();
                    if self.ch == Some('|') {
                        self.advance();
                        Ok(self.token(TokenKind::Or))
                    } else {
                        Err(LexerError::UnexpectedCharacter {
                            character: '|',
                            src: self.source.clone(),
                            span: (self.position, 1).into(),
                        })
                    }
                }
                Some('!') => Ok(self.token(TokenKind::Not)),
                Some('0'..='9') => self.read_number(),
                Some('a'..='z') | Some('A'..='Z') | Some('_') => self.read_identifier(),
                Some('"') => self.read_string(),
                Some(c) => Err(LexerError::UnexpectedCharacter {
                    character: c,
                    src: self.source.clone(),
                    span: (self.position, 1).into(),
                }),
                None => {
                    // Handle EOF: generate any pending dedents
                    if self.indent_stack.len() > 1 {
                        let indent_level = self.indent_stack.pop().unwrap();
                        return Ok(Token::new(
                            TokenKind::Dedent(indent_level),
                            self.position,
                            self.position,
                        ));
                    }
                    return Ok(Token::new(TokenKind::Eof, self.position, self.position));
                }
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

    fn read_string(&mut self) -> Result<Token, LexerError> {
        // Skip the opening quote
        let start = self.position;
        self.advance();

        let mut string = String::new();

        // Read until closing quote or EOF
        while let Some(ch) = self.ch {
            if ch == '"' {
                // Found the closing quote
                self.advance(); // Skip closing quote
                return Ok(Token::new(TokenKind::String(string), start, self.position));
            } else if ch == '\n' || ch == '\r' {
                // Strings can't contain newlines
                return Err(LexerError::UnterminatedString {
                    src: self.source.clone(),
                    span: (start, self.position - start).into(),
                });
            } else if ch == '\\' {
                // Handle escape sequences
                self.advance(); // Skip the backslash
                match self.ch {
                    Some('n') => string.push('\n'),
                    Some('r') => string.push('\r'),
                    Some('t') => string.push('\t'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    Some(c) => {
                        return Err(LexerError::InvalidEscapeSequence {
                            character: c,
                            src: self.source.clone(),
                            span: (self.position - 1, 2).into(),
                        });
                    }
                    None => {
                        return Err(LexerError::UnterminatedString {
                            src: self.source.clone(),
                            span: (start, self.position - start).into(),
                        });
                    }
                }
                self.advance();
            } else {
                // Add character to string and continue
                string.push(ch);
                self.advance();
            }
        }

        // If we get here, we reached EOF without finding a closing quote
        Err(LexerError::UnterminatedString {
            src: self.source.clone(),
            span: (start, self.position - start).into(),
        })
    }
}
