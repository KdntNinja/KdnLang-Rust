use crate::lexer::token::token_kind::TokenKind;
use crate::parser::ast::Statement;
use crate::parser::core::parser::Parser;
use crate::parser::error::ParserError;
use miette::Result;

pub fn parse_if_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // Consume 'if' token
    parser.advance_skipping_whitespace()?;

    // Parse condition
    let condition = Box::new(parser.parse_expression()?);

    // Parse then branch as a block
    let then_branch = Box::new(parser.parse_block()?);

    // Check for else branch
    let else_branch = if parser.current_token.as_ref().map_or(
        false,
        |t| matches!(t.kind(), TokenKind::Identifier(n) if n == "else"),
    ) {
        // Consume 'else'
        parser.advance_skipping_whitespace()?;

        // Parse the else block
        Some(Box::new(parser.parse_block()?))
    } else {
        None
    };

    Ok(Statement::If {
        condition,
        then_branch,
        else_branch,
    })
}
