use crate::lexer::token_kind::TokenKind;
use crate::parser::ast::Statement;
use crate::parser::core::parser::Parser;
use crate::parser::error::ParserError;
use miette::Result;

pub fn parse_while_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // Consume 'while' token
    parser.advance_skipping_whitespace()?;

    // Parse condition
    let condition = Box::new(parser.parse_expression()?);

    // Parse body as a block
    let body = Box::new(parser.parse_block()?);

    Ok(Statement::While { condition, body })
}

pub fn parse_for_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // Consume 'for' token
    parser.advance_skipping_whitespace()?;

    // Parse variable name
    let variable = match parser.current_token.as_ref().map(|t| t.kind()) {
        Some(TokenKind::Identifier(name)) => {
            let var_name = name.clone();
            parser.advance_skipping_whitespace()?;
            var_name
        }
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", parser.current_token.as_ref().map(|t| t.kind())),
                src: parser.source.clone(),
                span: parser
                    .current_token
                    .as_ref()
                    .map_or((0, 1).into(), |t| t._span()),
            });
        }
    };

    // Expect equals token
    parser.expect_token(TokenKind::Equals)?;

    // Parse start value expression
    let start_value = Box::new(parser.parse_expression()?);

    // Expect 'to' keyword
    match parser.current_token.as_ref().map(|t| t.kind()) {
        Some(TokenKind::Identifier(name)) if name == "to" => {
            parser.advance_skipping_whitespace()?;
        }
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "to".to_string(),
                found: format!("{:?}", parser.current_token.as_ref().map(|t| t.kind())),
                src: parser.source.clone(),
                span: parser
                    .current_token
                    .as_ref()
                    .map_or((0, 1).into(), |t| t._span()),
            });
        }
    }

    // Parse end value expression
    let end_value = Box::new(parser.parse_expression()?);

    // Parse body as a block
    let body = Box::new(parser.parse_block()?);

    Ok(Statement::ForLoop {
        variable,
        start_value,
        end_value,
        body,
    })
}
