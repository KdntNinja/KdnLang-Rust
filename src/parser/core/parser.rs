use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::lexer::token_kind::TokenKind;
use crate::parser::ast::{Expression, Program, Statement};
use crate::parser::error::ParserError;
use crate::parser::statements::{function_call, if_statement, loop_statements};
use miette::Result;

pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Option<Token>,
    pub source: String,
    pub current_indentation: usize,
    pub line_start: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, source: String) -> Result<Self, ParserError> {
        let mut parser = Self {
            lexer,
            current_token: None,
            source,
            current_indentation: 0,
            line_start: true,
        };

        // Initialize by getting the first token
        parser.advance()?;

        Ok(parser)
    }

    pub fn advance(&mut self) -> Result<(), ParserError> {
        self.current_token = match self.lexer.next_token() {
            Ok(token) => {
                // Track if we're at the start of a line
                match token.kind() {
                    TokenKind::Newline => {
                        self.line_start = true;
                    }
                    TokenKind::Whitespace(spaces) if self.line_start => {
                        // Count leading whitespace to determine indentation
                        self.current_indentation = *spaces;
                    }
                    _ if self.line_start => {
                        // First non-whitespace token on a line
                        self.line_start = false;
                    }
                    _ => {}
                }
                Some(token)
            }
            Err(err) => {
                return Err(ParserError::UnexpectedToken {
                    expected: "any token".to_string(),
                    found: format!("error: {}", err),
                    src: self.source.clone(),
                    span: (0, 1).into(),
                });
            }
        };
        Ok(())
    }

    // Add this method back for external modules to use
    pub fn advance_skipping_whitespace(&mut self) -> Result<(), ParserError> {
        self.advance()?;

        // Skip whitespace and newline tokens
        while self
            .current_token
            .as_ref()
            .map_or(false, |t| match t.kind() {
                TokenKind::Whitespace(_) | TokenKind::Newline => true,
                _ => false,
            })
        {
            self.advance()?;
        }

        Ok(())
    }

    pub fn check_token(&self, kind: TokenKind) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |t| t.kind() == &kind)
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        if let Some(token) = self.current_token.clone() {
            if token.kind() == &kind {
                self.advance()?;
                return Ok(token);
            } else {
                return Err(ParserError::UnexpectedToken {
                    expected: format!("{:?}", kind),
                    found: format!("{:?}", token.kind()),
                    src: self.source.clone(),
                    span: token._span(),
                });
            }
        }

        Err(ParserError::UnexpectedEOF {
            src: self.source.clone(),
            span: (self.source.len().saturating_sub(1), 1).into(),
        })
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();

        while !self.check_token(TokenKind::Eof) {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(ParserError::UnexpectedToken { found, .. }) if found == "EOF" => {
                    // We've reached EOF - this is fine, just stop parsing
                    break;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(Program { statements })
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        // Skip over any newlines and handle indentation tokens
        while self.check_token(TokenKind::Newline)
            || self.current_token.as_ref().map_or(false, |t| {
                matches!(t.kind(), TokenKind::Indent(_) | TokenKind::Dedent(_))
            })
        {
            // Just skip over these tokens - they're used for structure but don't form statements themselves
            self.advance()?;
        }

        // Check for EOF
        if self.check_token(TokenKind::Eof) {
            // If we hit EOF, that's fine - caller should handle it
            return Err(ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: "EOF".to_string(),
                src: self.source.clone(),
                span: (self.source.len().saturating_sub(1), 1).into(),
            });
        }

        match self.current_token.as_ref().map(|t| t.kind()) {
            Some(TokenKind::Identifier(name)) if name == "if" => {
                // Use the module function
                if_statement::parse_if_statement(self)
            }
            Some(TokenKind::Identifier(name)) if name == "for" => {
                // Use the module function
                loop_statements::parse_for_statement(self)
            }
            Some(TokenKind::Identifier(name)) if name == "while" => {
                // Use the module function
                loop_statements::parse_while_statement(self)
            }
            Some(TokenKind::Identifier(name)) if name == "print" => {
                let function_call = function_call::parse_function_call(self)?;
                Ok(Statement::FunctionCall {
                    name: function_call.0,
                    arguments: function_call.1,
                })
            }
            Some(TokenKind::Identifier(_)) => {
                // Check ahead to see if this is an assignment or function call
                let identifier_token = self.current_token.clone().unwrap();
                self.advance()?;

                if self.check_token(TokenKind::Equals) {
                    // This is an assignment
                    let identifier = match identifier_token.kind() {
                        TokenKind::Identifier(name) => name.clone(),
                        _ => unreachable!(),
                    };

                    // Consume equals token
                    self.advance()?;

                    // Parse expression
                    let expr = self.parse_expression()?;

                    Ok(Statement::Assignment {
                        identifier,
                        expression: Box::new(expr),
                    })
                } else if self.check_token(TokenKind::LeftParen) {
                    // This is a function call
                    // Step back to reparse as function call
                    unimplemented!("Function calls not fully implemented yet")
                } else {
                    Err(ParserError::UnexpectedToken {
                        expected: "= or (".to_string(),
                        found: format!("{:?}", self.current_token.as_ref().map(|t| t.kind())),
                        src: self.source.clone(),
                        span: self
                            .current_token
                            .as_ref()
                            .map_or((0, 1).into(), |t| t._span()),
                    })
                }
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "statement".to_string(),
                found: format!("{:?}", self.current_token.as_ref().map(|t| t.kind())),
                src: self.source.clone(),
                span: self
                    .current_token
                    .as_ref()
                    .map_or((0, 1).into(), |t| t._span()),
            }),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        use crate::parser::expressions::binary_ops;
        binary_ops::parse_comparison(self)
    }

    pub fn parse_block(&mut self) -> Result<Statement, ParserError> {
        let current_indent = self.current_indentation;

        // Skip any newlines before the block content
        while self.check_token(TokenKind::Newline) {
            self.advance()?;
        }

        // Parse statements in the block
        let mut statements = Vec::new();

        // Keep parsing statements until indentation decreases or EOF
        while !self.check_token(TokenKind::Eof) {
            // Check for indentation changes
            if self.line_start {
                // If indentation decreases below our level, we're done with this block
                if self.current_indentation < current_indent {
                    break;
                }

                // If indentation is the same as our current level, but the token is 'else',
                // we need to stop if we're inside an if block
                if self.current_indentation == current_indent
                    && self.current_token.as_ref().map_or(false, |t| {
                        if let TokenKind::Identifier(name) = t.kind() {
                            name == "else"
                        } else {
                            false
                        }
                    })
                {
                    break;
                }
            }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(ParserError::UnexpectedToken { found, .. }) if found == "EOF" => {
                    // We've reached EOF - this is fine, just stop parsing
                    break;
                }
                Err(err) => return Err(err),
            }
        }

        Ok(Statement::Block {
            statements,
            indentation: current_indent,
        })
    }
}
