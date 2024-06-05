use std::fmt;

use crate::token_type::*;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(x) => write!(f, "{x}"),
            Literal::String(x) => write!(f, "{x}"),
            Literal::Bool(x) => write!(f, "{x}"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token { ttype, lexeme, literal, line }
    }

    pub fn eof(line: usize) -> Token {
        Token { 
            ttype: TokenType::Eof, 
            lexeme: "".to_string(), 
            literal: None, 
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.ttype, self.lexeme, if let Some(literal) = &self.literal {
            literal.to_string()
        } else {
            "None".to_string()
        }, self.line)
    }
}