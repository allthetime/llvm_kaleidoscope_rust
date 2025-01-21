// src/parser/mod.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Binary(Box<Expr>, Token, Box<Expr>),
}

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: I,
    current_token: Option<Token>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(mut tokens: I) -> Self {
        let current_token = tokens.next();
        Parser {
            tokens,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }

    fn parse_number(&mut self) -> Expr {
        if let Some(Token::Number(value)) = self.current_token {
            self.advance();
            Expr::Number(value)
        } else {
            panic!("Expected a number");
        }
    }

    fn parse_paren_expr(&mut self) -> Expr {
        self.advance(); // consume '('
        let expr = self.parse_expression();
        if self.current_token != Some(Token::RParen) {
            panic!("Expected ')'");
        }
        self.advance(); // consume ')'
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token {
            Some(Token::Number(_)) => self.parse_number(),
            Some(Token::LParen) => self.parse_paren_expr(),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    let op = (*token).clone();
                    self.advance();
                    let right = self.parse_primary();
                    left = Expr::Binary(Box::new(left), op, Box::new(right));
                }
                _ => break,
            }
        }

        left
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }
}
