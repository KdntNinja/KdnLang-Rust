use pest::Parser;
use pest_derive::Parser;
use crate::token::Token;

// Define the pest parser for the language
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct KdnLangParser;

// Represents an expression in the language.
#[derive(Debug)]
pub enum Expr {
    // Represents a number literal in the expression.
    Number(i32),
    // Represents a binary operation with a left and right expression and an operator.
    BinaryOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
    // Represents an identifier in the expression.
    Identifier(String),
}

// The Parser struct is responsible for parsing tokens into an abstract syntax tree (AST).
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // Creates a new parser with the given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // Parses the tokens into an expression.
    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_expression()
    }

    // Parses an expression.
    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_term()
    }

    // Parses terms in the expression, handling addition and subtraction.
    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: '+',
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: '-',
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Some(expr)
    }

    // Parses factors in the expression, handling multiplication, division, and identifiers.
    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Asterisk => {
                    self.advance();
                    let right = self.parse_primary()?;
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: '*',
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary()?;
                    expr = Expr::BinaryOp {
                        left: Box::new(expr),
                        op: '/',
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Some(expr)
    }

    // Parses primary expressions, such as numbers, identifiers, and parenthesized expressions.
    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek()? {
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Some(Expr::Number(n))
            }
            Token::Identifier(id) => {
                let id = id.clone();
                self.advance();
                Some(Expr::Identifier(id))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression();
                if matches!(self.peek(), Some(Token::RightParen)) {
                    self.advance();
                }
                expr
            }
            _ => None,
        }
    }

    // Returns the current token without advancing the position.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Advances the position to the next token.
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
}
