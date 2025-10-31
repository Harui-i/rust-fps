use crate::tokenizer::Token;
use num_rational::BigRational;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(BigRational),
    Variable(char),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEof,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.consume() {
            Some(Token::Num(n)) => Ok(Expr::Num(n.clone())),
            Some(Token::Variable(c)) => Ok(Expr::Variable(*c)),
            Some(Token::LParen) => {
                let expr = self.parse_expr(0)?;
                match self.consume() {
                    Some(Token::RParen) => Ok(expr),
                    Some(t) => Err(ParserError::UnexpectedToken(t.clone())),
                    None => Err(ParserError::UnexpectedEof),
                }
            }
            Some(Token::Minus) => {
                let expr = self.parse_expr(5)?; // Unary minus precedence
                Ok(Expr::Neg(Box::new(expr)))
            }
            Some(t) => Err(ParserError::UnexpectedToken(t.clone())),
            None => Err(ParserError::UnexpectedEof),
        }
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ParserError> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Some(op) => op,
                None => break,
            };

            let (l_bp, r_bp) = match infix_binding_power(op) {
                Some(bp) => bp,
                None => break,
            };

            if l_bp < min_bp {
                break;
            }

            let op = self.consume().unwrap().clone();

            let rhs = self.parse_expr(r_bp)?;

            lhs = match op {
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                Token::Star => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                Token::Slash => Expr::Div(Box::new(lhs), Box::new(rhs)),
                Token::Caret => Expr::Pow(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };
        }

        Ok(lhs)
    }
}

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
        Token::Plus | Token::Minus => Some((1, 2)),
        Token::Star | Token::Slash => Some((3, 4)),
        Token::Caret => Some((6, 5)), // Right-associative
        _ => None,
    }
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse_expr(0)
}
