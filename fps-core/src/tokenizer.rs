use num_rational::BigRational;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionToken {
    Sin,
    Cos,
    Exp,
    Log,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(BigRational),
    Variable(char),
    Function(FunctionToken),
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
    #[error("Unexpected identifier: {0}")]
    UnexpectedIdentifier(String),
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
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphabetic() {
                        ident.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let token = match ident.as_str() {
                    "x" => Token::Variable('x'),
                    "sin" => Token::Function(FunctionToken::Sin),
                    "cos" => Token::Function(FunctionToken::Cos),
                    "exp" => Token::Function(FunctionToken::Exp),
                    "log" => Token::Function(FunctionToken::Log),
                    _ => return Err(TokenizerError::UnexpectedIdentifier(ident)),
                };
                tokens.push(token);
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
        let curr_starts_value = matches!(
            curr,
            Token::Num(_) | Token::Variable(_) | Token::LParen | Token::Function(_)
        );

        if prev_is_value && curr_starts_value {
            new_tokens.push(Token::Star);
        }
        new_tokens.push(curr.clone());
    }

    new_tokens
}
