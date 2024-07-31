use logos::Logos;
use std::collections::HashMap;

pub type Lexer<'l> = logos::Lexer<'l, Token<'l>>;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'t> {
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().trim_matches('"'))]
    String(&'t str),
}

#[derive(Debug)]
pub enum Value<'v> {
    String(&'v str),
    Object(HashMap<&'v str, Value<'v>>),
}

pub fn parse_value<'v>(lexer: &mut Lexer<'v>) -> Result<Value<'v>, String> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::BraceOpen) => parse_object(lexer),
            Ok(Token::String(str)) => Ok(Value::String(str)),
            _ => Err("Unexpected token".to_owned()),
        }
    } else {
        Err("Empty values are not allowed".to_owned())
    }
}

pub fn parse_object<'v>(lexer: &mut Lexer<'v>) -> Result<Value<'v>, String> {
    let mut map = HashMap::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::BraceClose) => return Ok(Value::Object(map)),
            Ok(Token::String(key)) => {
                let value = parse_value(lexer)?;
                map.insert(key, value);
            }
            _ => return Err("Unexpected token".to_owned()),
        }
    }

    Err("unmatched opening brace defined here".to_owned())
}

pub fn parse<'v>(lexer: &mut Lexer<'v>) -> Result<Value<'v>, String> {
    if let Some(token) = lexer.next() {
        match token {
            Ok(Token::String(top_key)) => parse_value(lexer).map(|v| {
                let mut map = HashMap::new();
                map.insert(top_key, v);
                Value::Object(map)
            }),
            _ => Err("Expected key".to_owned()),
        }
    } else {
        Err("Empty vdf".to_owned())
    }
}
