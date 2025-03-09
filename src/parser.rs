use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    BinaryOp {
        left: Box<Expr>,
        op: char,
        right: Box<Expr>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Option<Expr> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while let Some(Token::Plus) = self.peek() {
            self.advance();
            let right = self.parse_factor()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: '+',
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;

        while let Some(Token::Asterisk) = self.peek() {
            self.advance();
            let right = self.parse_primary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: '*',
                right: Box::new(right),
            };
        }

        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek()? {
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Some(Expr::Number(n))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression();
                if matches!(self.peek(), Some(Token::RightParen)) {
                    self.advance();
                }
                expr
            }
            _ => None,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
}
