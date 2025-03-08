use crate::lexer::core::lexer::Lexer;
use crate::lexer::token::token::Token;
use crate::lexer::token::token_kind::TokenKind;

pub fn handle_newline(lexer: &mut Lexer) -> Token {
    let pos = lexer.position;
    lexer.advance();

    // Handle Windows-style CRLF
    if lexer.ch == Some('\n')
        && lexer.position > 0
        && lexer.input.get(lexer.position - 2) == Some(&'\r')
    {
        lexer.advance();
    }

    lexer.line_start = true;
    Token::new(TokenKind::Newline, pos, lexer.position)
}

pub fn skip_whitespace(lexer: &mut Lexer) {
    while lexer
        .ch
        .map_or(false, |c| c.is_whitespace() && c != '\n' && c != '\r')
    {
        lexer.advance();
    }
}
