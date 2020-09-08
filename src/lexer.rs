// use std::ops::DerefMut;
use std::{iter::Peekable, str::Chars};

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Word(String),
    Number(f64),
    Eof,
}

pub type LexResult = anyhow::Result<Token>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Box<Peekable<Chars<'a>>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: Box::new(input.chars().peekable()),
            position: 0,
        }
    }

    pub fn lex(&mut self) -> LexResult {
        let chars = &mut self.chars;
        let mut position = self.position;
        let source = self.input;

        loop {
            let c = chars.peek();

            if c.is_none() {
                self.position = position;
                return Ok(Token::Eof);
            }

            if c.unwrap().is_whitespace() {
                position += 1;
                chars.next();
            } else {
                break;
            }
        }

        let next = chars.next();

        if next.is_none() {
            return Ok(Token::Eof);
        }

        let start = position;
        position += 1;
        let next = next.unwrap();

        let result = match next {
            '0'..='9' => {
                loop {
                    let c = match chars.peek() {
                        Some(c) => *c,
                        None => return Ok(Token::Number(next.to_string().parse().unwrap())),
                    };

                    if c.is_whitespace() || c != '.' && !c.is_digit(10) {
                        break;
                    }

                    chars.next();
                    position += 1;
                }
                self.position = position;
                Ok(Token::Number(source[start..position].parse().unwrap()))
            }

            _ => {
                loop {
                    let c = match chars.peek() {
                        Some(c) => *c,
                        None => return Ok(Token::Word(next.to_string())),
                    };

                    if c.is_whitespace() {
                        break;
                    }

                    chars.next();
                    position += 1;
                }

                self.position = position;
                Ok(Token::Word(source[start..position].to_string()))
            }
        };

        result
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(Token::Eof) | Err(_) => None,
            Ok(token) => Some(token),
        }
    }
}
