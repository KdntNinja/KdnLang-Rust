use crate::lexer::token_kind::TokenKind;
use crate::parser::ast::Expression;
use crate::parser::core::parser::Parser;
use crate::parser::error::ParserError;
use miette::Result;

pub fn parse_primary(parser: &mut Parser) -> Result<Expression, ParserError> {
    if let Some(token) = parser.current_token.clone() {
        match token.kind() {
            TokenKind::Number(n) => {
                parser.advance_skipping_whitespace()?;
                Ok(Expression::Number(*n))
            }
            TokenKind::String(s) => {
                parser.advance_skipping_whitespace()?;
                Ok(Expression::String(s.clone()))
            }
            TokenKind::Identifier(name) => {
                // Look ahead to see if this is a function call
                parser.advance_skipping_whitespace()?;

                if parser.check_token(TokenKind::LeftParen) {
                    // Function call
                    let name_clone = name.clone();

                    // Consume left paren
                    parser.advance_skipping_whitespace()?;

                    // Parse arguments
                    let mut arguments = Vec::new();

                    if !parser.check_token(TokenKind::RightParen) {
                        // Parse first argument
                        arguments.push(parser.parse_expression()?);

                        // Parse remaining arguments
                        while parser.check_token(TokenKind::Comma) {
                            parser.advance_skipping_whitespace()?; // Consume comma
                            arguments.push(parser.parse_expression()?);
                        }
                    }

                    // Expect right parenthesis
                    parser.expect_token(TokenKind::RightParen)?;

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
                parser.advance_skipping_whitespace()?;
                let expr = parser.parse_expression()?;
                parser.expect_token(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: "expression".to_string(),
                found: format!("{:?}", token.kind()),
                src: parser.source.clone(),
                span: token._span(),
            }),
        }
    } else {
        Err(ParserError::UnexpectedEOF {
            src: parser.source.clone(),
            span: (parser.source.len().saturating_sub(1), 1).into(),
        })
    }
}
