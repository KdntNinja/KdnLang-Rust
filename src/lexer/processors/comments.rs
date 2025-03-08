use crate::lexer::core::lexer::Lexer;

pub fn skip_comment(lexer: &mut Lexer) {
    // Skip the '#' character
    lexer.advance();

    // Skip all characters until a newline or EOF
    while lexer.ch.is_some() && lexer.ch != Some('\n') && lexer.ch != Some('\r') {
        lexer.advance();
    }
}
