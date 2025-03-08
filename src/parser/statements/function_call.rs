use crate::lexer::token_kind::TokenKind;
use crate::parser::ast::Expression;
use crate::parser::core::parser::Parser;
use crate::parser::error::ParserError;
use miette::Result;

pub fn parse_function_call(parser: &mut Parser) -> Result<(String, Vec<Expression>), ParserError> {
    // Get function name
    let name = match parser.current_token.as_ref().map(|t| t.kind()) {
        Some(TokenKind::Identifier(name)) => name.clone(),
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "function name".to_string(),
                found: format!("{:?}", parser.current_token.as_ref().map(|t| t.kind())),
                src: parser.source.clone(),
                span: parser
                    .current_token
                    .as_ref()
                    .map_or((0, 1).into(), |t| t._span()),
            });
        }
    };

    // Consume function name token
    parser.advance_skipping_whitespace()?;

    // Expect left parenthesis
    parser.expect_token(TokenKind::LeftParen)?;

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

    Ok((name, arguments))
}
