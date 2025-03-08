use crate::lexer::token::token_kind::TokenKind;
use crate::parser::ast::{BinaryOperator, Expression};
use crate::parser::core::parser::Parser;
use crate::parser::error::ParserError;
use crate::parser::expressions::primary;
use miette::Result;

pub fn parse_comparison(parser: &mut Parser) -> Result<Expression, ParserError> {
    let mut left = parse_additive(parser)?;

    while let Some(token) = parser.current_token.clone() {
        match token.kind() {
            TokenKind::GreaterThan => {
                parser.advance_skipping_whitespace()?;
                let right = parse_additive(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::GreaterThan,
                    right: Box::new(right),
                };
            }
            TokenKind::GreaterThanEquals => {
                parser.advance_skipping_whitespace()?;
                let right = parse_additive(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::GreaterThanEquals,
                    right: Box::new(right),
                };
            }
            TokenKind::LessThan => {
                parser.advance_skipping_whitespace()?;
                let right = parse_additive(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::LessThan,
                    right: Box::new(right),
                };
            }
            TokenKind::LessThanEquals => {
                parser.advance_skipping_whitespace()?;
                let right = parse_additive(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::LessThanEquals,
                    right: Box::new(right),
                };
            }
            TokenKind::DoubleEquals => {
                // Use DoubleEquals for equality comparison
                parser.advance_skipping_whitespace()?;
                let right = parse_additive(parser)?;
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

pub fn parse_additive(parser: &mut Parser) -> Result<Expression, ParserError> {
    let mut left = parse_multiplicative(parser)?;

    while let Some(token) = parser.current_token.clone() {
        match token.kind() {
            TokenKind::Plus => {
                parser.advance_skipping_whitespace()?;
                let right = parse_multiplicative(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::Add,
                    right: Box::new(right),
                };
            }
            TokenKind::Minus => {
                parser.advance_skipping_whitespace()?;
                let right = parse_multiplicative(parser)?;
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

pub fn parse_multiplicative(parser: &mut Parser) -> Result<Expression, ParserError> {
    let mut left = primary::parse_primary(parser)?;

    while let Some(token) = parser.current_token.clone() {
        match token.kind() {
            TokenKind::Asterisk => {
                parser.advance_skipping_whitespace()?;
                let right = primary::parse_primary(parser)?;
                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(right),
                };
            }
            TokenKind::Slash => {
                parser.advance_skipping_whitespace()?;
                let right = primary::parse_primary(parser)?;
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
