use crate::lexer::token::token_kind::TokenKind;
use miette::SourceSpan;

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: (start, end - start).into(),
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn _span(&self) -> SourceSpan {
        self.span
    }
}
