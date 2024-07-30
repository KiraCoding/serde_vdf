use logos::{Lexer, Logos};
use std::collections::HashMap;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r" \t\n\f]+")]
enum Token {
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("\"")]
    Quote,

    #[token("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[token("[0-9]+")]
    Number,
}

pub struct Parser<'p> {
    lexer: Lexer<'p, Token>,
}

impl<'p> Parser<'p> {
    pub fn new(input: &'p str) -> Self {
        Self {
            lexer: Token::lexer(input),
        }
    }

    pub fn parse(&mut self) -> Result<Vdf, String> {
        self.parse_vdf()
    }

    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        match self.lexer.next().unwrap().ok() {
            Some(token) if token == expected => Ok(token),
            Some(token) => Err(format!("Expected {:?}, found {:?}", expected, token)),
            None => Err(format!("Expected {:?}, but the input has ended", expected)),
        }
    }

    fn parse_vdf(&mut self) -> Result<Vdf, String> {
        let mut data = HashMap::new();

        while let Some(token) = self.lexer.next().unwrap().ok() {
            match token {
                Token::BraceOpen => {
                    let key = self.parse_identifier()?;
                    let value = self.parse_group()?;
                    data.insert(key, VdfEntry::Group(value));
                }
                Token::Quote => {
                    let key = self.parse_identifier()?;
                    let value = self.parse_value()?;
                    data.insert(key, VdfEntry::Value(value));
                }
                Token::BraceClose => {
                    return Ok(Vdf { data });
                }
                _ => return Err("Unexpected token".into()),
            }
        }

        Err("Unexpected end of input".into())
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        match self.lexer.next() {
            Some(Ok(Token::Identifier)) => Ok(self.lexer.slice().to_string()),
            _ => Err("Expected identifier".into()),
        }
    }

    fn parse_value(&mut self) -> Result<String, String> {
        match self.lexer.next() {
            Some(Ok(Token::Quote)) => {
                let value = self.lexer.slice().to_string();
                match self.lexer.next() {
                    Some(Ok(Token::Quote)) => Ok(value),
                    _ => Err("Expected closing quote".into()),
                }
            }
            _ => Err("Expected value".into()),
        }
    }

    fn parse_group(&mut self) -> Result<Vdf, String> {
        let mut data = HashMap::new();

        while let Some(token) = self.lexer.next().unwrap().ok() {
            match token {
                Token::BraceOpen => {
                    let key = self.parse_identifier()?;
                    let value = self.parse_group()?;
                    data.insert(key, VdfEntry::Group(value));
                }
                Token::Quote => {
                    let key = self.parse_identifier()?;
                    let value = self.parse_value()?;
                    data.insert(key, VdfEntry::Value(value));
                }
                Token::BraceClose => {
                    return Ok(Vdf { data });
                }
                _ => return Err("Unexpected token".into()),
            }
        }

        Err("Unexpected end of input".into())
    }
}

#[derive(Debug)]
pub struct Vdf {
    data: HashMap<String, VdfEntry>,
}

#[derive(Debug)]
pub enum VdfEntry {
    Value(String),
    Group(Vdf),
}

// "ParentKey1"
// {
// 	"ValueKey1"	"1"
// 	"ParentKey2"
// 	{
// 		...
// 	}
// }
