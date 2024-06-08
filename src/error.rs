use crate::token::*;
use crate::token_type::*;
#[derive(Debug)]
pub struct JialoxError {
    token: Option<Token>,
    line: usize,
    message: String,
}

impl JialoxError {
    pub fn error(line: usize, message: &str) -> JialoxError {
        let err = JialoxError {
            token: None,
            line,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn parse_error(token: &Token, message: &str) -> JialoxError {
        let err = JialoxError {
            token: Some(token.mirror()),
            line: token.line(),
            message: message.to_string(),
        };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
        if let Some(token) = &self.token {
            if token.is(TokenType::Eof) {
                eprintln!("{} at end {}", token.line(), self.message);
            } else {
                eprintln!("{} at '{}' {}", token.line(), token.lexeme(), self.message);
            }
        }
    }
}
