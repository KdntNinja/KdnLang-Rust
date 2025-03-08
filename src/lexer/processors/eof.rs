use crate::lexer::core::lexer::Lexer;
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;

pub fn handle_eof(lexer: &mut Lexer) -> Token {
    // Handle EOF: generate any pending dedents
    if lexer.indent_stack.len() > 1 {
        let indent_level = lexer.indent_stack.pop().unwrap();
        Token::new(
            TokenKind::Dedent(indent_level),
            lexer.position,
            lexer.position,
        )
    } else {
        Token::new(TokenKind::Eof, lexer.position, lexer.position)
    }
}
