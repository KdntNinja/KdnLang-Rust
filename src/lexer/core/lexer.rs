use crate::lexer::error::LexerError;
use crate::lexer::processors::comments::skip_comment;
use crate::lexer::processors::eof::handle_eof;
use crate::lexer::processors::indentation::handle_indentation;
use crate::lexer::processors::literals::{read_identifier, read_number, read_string};
use crate::lexer::processors::operators::read_operator;
use crate::lexer::processors::whitespace::{handle_newline, skip_whitespace};
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;
use miette::Result;
use std::collections::VecDeque;

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
    pub source: String,
    pub line_start: bool,
    pub indent_stack: Vec<usize>,
    pub pending_tokens: VecDeque<Token>,
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

    pub fn advance(&mut self) {
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
                let indent_start = self.position;

                match handle_indentation(self, indent_start)? {
                    Some(token) => return Ok(token),
                    None => {}
                }
            }

            match self.ch {
                // Handle newlines
                Some('\n') | Some('\r') => {
                    return Ok(handle_newline(self));
                }

                // Skip whitespace
                Some(ch) if ch.is_whitespace() => {
                    skip_whitespace(self);
                    continue;
                }

                // Skip comments
                Some('#') => {
                    skip_comment(self);
                    continue;
                }

                // Handle literals and operators
                Some('0'..='9') => return read_number(self),
                Some('a'..='z') | Some('A'..='Z') | Some('_') => return read_identifier(self),
                Some('"') => return read_string(self),
                Some(_) => return read_operator(self),

                // Handle EOF
                None => {
                    return Ok(handle_eof(self));
                }
            }
        }
    }

    pub fn token(&mut self, kind: TokenKind) -> Token {
        let tok = Token::new(kind, self.position, self.read_position);
        self.advance();
        tok
    }
}
