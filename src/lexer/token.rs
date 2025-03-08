use crate::lexer::token_kind::TokenKind;
use miette::SourceSpan;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Token { kind, start, end }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> SourceSpan {
        (self.start, self.end - self.start).into()
    }
}