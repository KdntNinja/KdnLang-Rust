use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::lexer::token_kind::TokenKind;
use crate::parser::ast::{BinaryOperator, Expression, Program, Statement};
use crate::parser::error::ParserError;
use miette::Result;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Token>,
    source: String,
    #[allow(dead_code)]
    indentation_stack: Vec<usize>,
    current_indentation: usize,
    line_start: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, source: String) -> Result<Self, ParserError> {
        let mut parser = Self {
            lexer,
            current_token: None,
            source,
            indentation_stack: vec![0], // Start with base indentation level
            current_indentation: 0,
            line_start: true,
        };

        // Initialize by skipping whitespace tokens
        parser.advance_skipping_whitespace()?;

        Ok(parser)
    }

    fn advance(&mut self) -> Result<(), ParserError> {
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
                    span: (0, 1).into(), // This is a placeholder, ideally we'd get the position from the error
                });
            }
        };
        Ok(())
    }

    // Add a new helper to skip whitespace tokens
    fn advance_skipping_whitespace(&mut self) -> Result<(), ParserError> {
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

    fn check_token(&self, kind: TokenKind) -> bool {
        self.current_token
            .as_ref()
            .map_or(false, |t| t.kind() == &kind)
    }

    fn expect_token(&mut self, kind: TokenKind) -> Result<Token, ParserError> {
        if let Some(token) = self.current_token.clone() {
            if token.kind() == &kind {
                self.advance_skipping_whitespace()?;
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
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.as_ref().map(|t| t.kind()) {
            Some(TokenKind::Identifier(name)) if name == "for" => self.parse_for_statement(),
            Some(TokenKind::Identifier(name)) if name == "while" => self.parse_while_statement(),
            Some(TokenKind::Identifier(name)) if name == "print" => {
                // Fixed: Create a FunctionCall statement directly
                let function_call = self.parse_function_call()?;
                Ok(Statement::FunctionCall {
                    name: function_call.0,
                    arguments: function_call.1,
                })
            }
            Some(TokenKind::Identifier(_)) => {
                // Check ahead to see if this is an assignment or function call
                let identifier_token = self.current_token.clone().unwrap();
                self.advance_skipping_whitespace()?;

                if self.check_token(TokenKind::Equals) {
                    // This is an assignment
                    let identifier = match identifier_token.kind() {
                        TokenKind::Identifier(name) => name.clone(),
                        _ => unreachable!(),
                    };

                    // Consume equals token
                    self.advance_skipping_whitespace()?;

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

    fn parse_for_statement(&mut self) -> Result<Statement, ParserError> {
        // Consume 'for' token
        self.advance_skipping_whitespace()?;

        // Save current indentation level
        let current_indent = self.current_indentation;

        // Parse variable name
        let variable = match self.current_token.as_ref().map(|t| t.kind()) {
            Some(TokenKind::Identifier(name)) => {
                let var_name = name.clone();
                self.advance_skipping_whitespace()?;
                var_name
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", self.current_token.as_ref().map(|t| t.kind())),
                    src: self.source.clone(),
                    span: self
                        .current_token
                        .as_ref()
                        .map_or((0, 1).into(), |t| t._span()),
                });
            }
        };

        // Expect equals token
        self.expect_token(TokenKind::Equals)?;

        // Parse start value expression
        let start_value = Box::new(self.parse_expression()?);

        // Expect 'to' keyword
        match self.current_token.as_ref().map(|t| t.kind()) {
            Some(TokenKind::Identifier(name)) if name == "to" => {
                self.advance_skipping_whitespace()?;
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "to".to_string(),
                    found: format!("{:?}", self.current_token.as_ref().map(|t| t.kind())),
                    src: self.source.clone(),
                    span: self
                        .current_token
                        .as_ref()
                        .map_or((0, 1).into(), |t| t._span()),
                });
            }
        }

        // Parse end value expression
        let end_value = Box::new(self.parse_expression()?);

        // Parse statements in the for body
        // Expect the next statement to be indented more than the current indentation
        let mut body = Vec::new();

        // Keep parsing statements at a higher indentation level
        while !self.check_token(TokenKind::Eof) {
            // If we encounter a line with less indentation, we've exited the for block
            if self.line_start && self.current_indentation <= current_indent {
                break;
            }

            body.push(self.parse_statement()?);
        }

        Ok(Statement::ForLoop {
            variable,
            start_value,
            end_value,
            body,
        })
    }

    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        // Consume 'while' token
        self.advance_skipping_whitespace()?;

        // Save current indentation level
        let current_indent = self.current_indentation;

        // Parse condition
        let condition = Box::new(self.parse_expression()?);

        // Parse statements in the while body
        let mut body = Vec::new();

        // Keep parsing statements at a higher indentation level
        while !self.check_token(TokenKind::Eof) {
            // If we encounter a line with less indentation, we've exited the while block
            if self.line_start && self.current_indentation <= current_indent {
                break;
            }

            body.push(self.parse_statement()?);
        }

        Ok(Statement::While { condition, body })
    }

    fn parse_function_call(&mut self) -> Result<(String, Vec<Expression>), ParserError> {
        // Get function name
        let name = match self.current_token.as_ref().map(|t| t.kind()) {
            Some(TokenKind::Identifier(name)) => name.clone(),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "function name".to_string(),
                    found: format!("{:?}", self.current_token.as_ref().map(|t| t.kind())),
                    src: self.source.clone(),
                    span: self
                        .current_token
                        .as_ref()
                        .map_or((0, 1).into(), |t| t._span()),
                });
            }
        };

        // Consume function name token
        self.advance_skipping_whitespace()?;

        // Expect left parenthesis
        self.expect_token(TokenKind::LeftParen)?;

        // Parse arguments
        let mut arguments = Vec::new();

        if !self.check_token(TokenKind::RightParen) {
            // Parse first argument
            arguments.push(self.parse_expression()?);

            // Parse remaining arguments
            while self.check_token(TokenKind::Comma) {
                self.advance_skipping_whitespace()?; // Consume comma
                arguments.push(self.parse_expression()?);
            }
        }

        // Expect right parenthesis
        self.expect_token(TokenKind::RightParen)?;

        Ok((name, arguments))
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_additive()?;

        while let Some(token) = self.current_token.clone() {
            match token.kind() {
                TokenKind::GreaterThan => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_additive()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::GreaterThan,
                        right: Box::new(right),
                    };
                }
                TokenKind::LessThan => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_additive()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::LessThan,
                        right: Box::new(right),
                    };
                }
                TokenKind::Equals => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_additive()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::Equals,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_multiplicative()?;

        while let Some(token) = self.current_token.clone() {
            match token.kind() {
                TokenKind::Plus => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_multiplicative()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::Add,
                        right: Box::new(right),
                    };
                }
                TokenKind::Minus => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_multiplicative()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current_token.clone() {
            match token.kind() {
                TokenKind::Asterisk => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(right),
                    };
                }
                TokenKind::Slash => {
                    self.advance_skipping_whitespace()?;
                    let right = self.parse_primary()?;
                    left = Expression::BinaryOp {
                        left: Box::new(left),
                        operator: BinaryOperator::Divide,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = self.current_token.clone() {
            match token.kind() {
                TokenKind::Number(n) => {
                    self.advance_skipping_whitespace()?;
                    Ok(Expression::Number(*n))
                }
                TokenKind::Identifier(name) => {
                    // Look ahead to see if this is a function call
                    self.advance_skipping_whitespace()?;

                    if self.check_token(TokenKind::LeftParen) {
                        // Function call
                        let name_clone = name.clone();

                        // Consume left paren
                        self.advance_skipping_whitespace()?;

                        // Parse arguments
                        let mut arguments = Vec::new();

                        if !self.check_token(TokenKind::RightParen) {
                            // Parse first argument
                            arguments.push(self.parse_expression()?);

                            // Parse remaining arguments
                            while self.check_token(TokenKind::Comma) {
                                self.advance_skipping_whitespace()?; // Consume comma
                                arguments.push(self.parse_expression()?);
                            }
                        }

                        // Expect right parenthesis
                        self.expect_token(TokenKind::RightParen)?;

                        Ok(Expression::FunctionCall {
                            name: name_clone,
                            arguments,
                        })
                    } else {
                        // Simple identifier
                        Ok(Expression::Identifier(name.clone()))
                    }
                }
                TokenKind::LeftParen => {
                    self.advance_skipping_whitespace()?;
                    let expr = self.parse_expression()?;
                    self.expect_token(TokenKind::RightParen)?;
                    Ok(expr)
                }
                _ => Err(ParserError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: format!("{:?}", token.kind()),
                    src: self.source.clone(),
                    span: token._span(),
                }),
            }
        } else {
            Err(ParserError::UnexpectedEOF {
                src: self.source.clone(),
                span: (self.source.len().saturating_sub(1), 1).into(),
            })
        }
    }
}
