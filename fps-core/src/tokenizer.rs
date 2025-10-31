use num_rational::BigRational;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(BigRational),
    Variable(char),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TokenizerError {
    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) {
                        num_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num = BigRational::from_integer(num_str.parse().unwrap());
                tokens.push(Token::Num(num));
            }
            'x' => {
                tokens.push(Token::Variable('x'));
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => return Err(TokenizerError::UnexpectedChar(c)),
        }
    }

    Ok(insert_implicit_stars(tokens))
}

fn insert_implicit_stars(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::new();
    if tokens.is_empty() {
        return new_tokens;
    }

    new_tokens.push(tokens[0].clone());

    for i in 1..tokens.len() {
        let prev = &tokens[i - 1];
        let curr = &tokens[i];

        let prev_is_value = matches!(prev, Token::Num(_) | Token::Variable(_) | Token::RParen);
        let curr_is_value = matches!(curr, Token::Num(_) | Token::Variable(_) | Token::LParen);

        if prev_is_value && curr_is_value {
            new_tokens.push(Token::Star);
        }
        new_tokens.push(curr.clone());
    }

    new_tokens
}
