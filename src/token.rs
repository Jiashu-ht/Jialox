use std::fmt;

use crate::literal::*;
use crate::token_type::*;

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }

    pub fn is(&self, ttype: TokenType) -> bool {
        self.ttype == ttype
    }

    pub fn mirror(&self) -> Token {
        Token {
            ttype: self.ttype,
            lexeme: self.lexeme.clone(),
            literal: self.literal.clone(),
            line: self.line,
        }
    }

    pub fn literal(&self) -> Option<Literal> {
        self.literal.clone()
    }

    // pub fn as_string(&self) -> String {
    //     self.lexeme.clone()
    // }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn ttype(&self) -> TokenType {
        self.ttype
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            },
            self.line
        )
    }
}
