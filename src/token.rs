use std::fmt;

use crate::token_type::*;

pub enum LiteralType {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::Number(x) => write!(f, "{x}"),
            LiteralType::String(x) => write!(f, "{x}"),
            LiteralType::Bool(x) => write!(f, "{x}"),
            LiteralType::Nil => write!(f, "nil"),
        }
    }
}

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<LiteralType>, line: usize) -> Token {
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