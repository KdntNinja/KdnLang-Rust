//! Parser for KdnLang - converts tokens to AST using Pest.

use crate::token::Token;
use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

/// KdnLang parser using Pest.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct KdnLangParser {
    tokens: Vec<Token>,
}

/// AST expression node types for KdnLang.
#[derive(Debug)]
pub enum Expr {
    /// Number literal
    Number(i32),
    
    /// Binary operation (math)
    BinaryOp {
        /// Left operand
        left: Box<Expr>,
        /// Operator (+, -, *, /)
        op: char,
        /// Right operand
        right: Box<Expr>,
    },
    
    /// Variable reference
    Identifier(String),
}

impl KdnLangParser {
    /// Creates new parser with given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    /// Parses tokens into AST, returns None if parsing fails.
    pub fn parse(&mut self) -> Option<Expr> {
        // Convert tokens to a string representation that Pest can parse
        let input = self.tokens_to_input();
        
        // Parse using Pest
        match Self::parse(Rule::program, &input) {
            Ok(pairs) => {
                // Convert parse result to AST
                self.build_ast(pairs)
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
                None
            }
        }
    }

    /// Converts tokens to a string representation for Pest
    fn tokens_to_input(&self) -> String {
        let mut input = String::new();
        for token in &self.tokens {
            match token {
                Token::Number(n) => input.push_str(&n.to_string()),
                Token::Plus => input.push('+'),
                Token::Minus => input.push('-'),
                Token::Asterisk => input.push('*'),
                Token::Slash => input.push('/'),
                Token::LeftParen => input.push('('),
                Token::RightParen => input.push(')'),
                Token::Identifier(s) => input.push_str(s),
                Token::Unknown(c) => input.push(*c),
            }
            // Add space between tokens to ensure proper separation
            input.push(' ');
        }
        input
    }

    /// Builds an AST from the Pest parse result
    fn build_ast(&self, mut pairs: Pairs<Rule>) -> Option<Expr> {
        // Get the program rule
        let program = pairs.next()?;
        
        // Get the expression rule
        let expr_pair = program.into_inner().next()?;
        
        // Build AST from expression
        Some(self.build_expr(expr_pair))
    }

    /// Recursively builds an AST from a Pest expression pair
    fn build_expr(&self, pair: Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::expr => {
                let mut pairs = pair.into_inner();
                let mut left = self.build_expr(pairs.next().unwrap());
                
                // Process any binary operations
                while let Some(op_pair) = pairs.next() {
                    let op = op_pair.as_str().chars().next().unwrap();
                    let right = self.build_expr(pairs.next().unwrap());
                    
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                
                left
            },
            Rule::term => {
                let mut pairs = pair.into_inner();
                let mut left = self.build_expr(pairs.next().unwrap());
                
                // Process any binary operations
                while let Some(op_pair) = pairs.next() {
                    let op = op_pair.as_str().chars().next().unwrap();
                    let right = self.build_expr(pairs.next().unwrap());
                    
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    };
                }
                
                left
            },
            Rule::factor => {
                let inner = pair.into_inner().next().unwrap();
                self.build_expr(inner)
            },
            Rule::primary => {
                let inner = pair.into_inner().next().unwrap();
                self.build_expr(inner)
            },
            Rule::number => {
                Expr::Number(pair.as_str().parse().unwrap())
            },
            Rule::identifier => {
                Expr::Identifier(pair.as_str().to_string())
            },
            _ => panic!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }
}

/// Grammar rules defined in the pest file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    program,
    expr,
    term,
    factor,
    primary,
    number,
    identifier,
    add_op,
    mul_op,
    WHITESPACE,
}
