use crate::lexer::core::lexer::Lexer;
use crate::lexer::error::LexerError;
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;
use miette::Result;

pub fn handle_indentation(
    lexer: &mut Lexer,
    indent_start: usize,
) -> Result<Option<Token>, LexerError> {
    let mut indent_size = 0;

    // Count spaces
    while lexer.ch == Some(' ') {
        indent_size += 1;
        lexer.advance();
    }

    // If we're at a newline or comment after spaces, just continue
    if lexer.ch == Some('\n') || lexer.ch == Some('\r') || lexer.ch == Some('#') {
        return Ok(None);
    }

    // We're no longer at the start of a line
    lexer.line_start = false;

    // Compare with the current indentation level
    let current_indent = *lexer.indent_stack.last().unwrap_or(&0);

    if indent_size > current_indent {
        // Indentation increased
        lexer.indent_stack.push(indent_size);
        return Ok(Some(Token::new(
            TokenKind::Indent(indent_size),
            indent_start,
            lexer.position,
        )));
    } else if indent_size < current_indent {
        // Indentation decreased, may need multiple dedents
        let mut dedents = Vec::new();
        while !lexer.indent_stack.is_empty() && indent_size < *lexer.indent_stack.last().unwrap() {
            let prev_indent = lexer.indent_stack.pop().unwrap();
            dedents.push(prev_indent);
        }

        // If the indentation doesn't match any previous level, it's an error
        if indent_size != *lexer.indent_stack.last().unwrap_or(&0) {
            return Err(LexerError::IndentationError {
                src: lexer.source.clone(),
                span: (indent_start, lexer.position - indent_start).into(),
            });
        }

        // Queue up dedent tokens
        for indent in dedents.iter().skip(1) {
            lexer.pending_tokens.push_back(Token::new(
                TokenKind::Dedent(*indent),
                indent_start,
                lexer.position,
            ));
        }

        // Return the first dedent
        return Ok(Some(Token::new(
            TokenKind::Dedent(*dedents.first().unwrap_or(&0)),
            indent_start,
            lexer.position,
        )));
    }

    // If indentation is exactly the same, continue normal processing
    Ok(None)
}
