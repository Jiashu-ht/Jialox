use std::collections::HashMap;

use crate::error::*;
use crate::literal::*;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let mut keywords = HashMap::new();
        Scanner::init_keywords(&mut keywords);
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    fn init_keywords(keywords: &mut HashMap<String, TokenType>) {
        keywords.insert("add".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("func".to_string(), TokenType::Func);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, JialoxError> {
        let mut had_error = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                    had_error = Some(e);
                }
            }
        }
        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), JialoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_terminator(TokenType::LeftParen),
            ')' => self.add_terminator(TokenType::RightParen),
            '{' => self.add_terminator(TokenType::LeftBrace),
            '}' => self.add_terminator(TokenType::RightBrace),
            ',' => self.add_terminator(TokenType::Comma),
            '.' => self.add_terminator(TokenType::Dot),
            '-' => self.add_terminator(TokenType::Minus),
            '+' => self.add_terminator(TokenType::Plus),
            ';' => self.add_terminator(TokenType::Semicolon),
            '*' => self.add_terminator(TokenType::Star),
            '!' => {
                let tt = if self.next_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_terminator(tt);
            }
            '=' => {
                let tt = if self.next_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_terminator(tt);
            }
            '<' => {
                let tt = if self.next_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_terminator(tt);
            }
            '>' => {
                let tt = if self.next_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_terminator(tt);
            }
            '/' => {
                if self.next_match('/') {
                    // A comment goes until the end of the line
                    while let Some(ch) = self.currentc() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else if self.next_match('*') {
                    // block
                    self.scan_block_comments()?;
                } else {
                    self.add_terminator(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number();
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            }
            _ => {
                return Err(JialoxError::error(self.line, "Unexpected character"));
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let result = self.source.get(self.current).copied().unwrap();
        self.current += 1;
        result
    }

    fn add_terminator(&mut self, ttype: TokenType) {
        self.add_token(ttype, None);
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line));
    }

    fn next_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current).copied() {
            Some(ch) if ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn currentc(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn string(&mut self) -> Result<(), JialoxError> {
        while let Some(ch) = self.currentc() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(JialoxError::error(self.line, "Unterminated string."));
        }
        self.advance();
        // TODO: handle escape sequences
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::String, Some(Literal::Str(value)));
        Ok(())
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.currentc()) {
            self.advance();
        }

        if Some('.') == self.currentc() && Scanner::is_digit(self.next_currentc()) {
            self.advance();
            while Scanner::is_digit(self.currentc()) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token(TokenType::Number, Some(Literal::Num(num)));
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some('0'..='9') = ch {
            return true;
        }
        false
    }

    fn next_currentc(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.currentc()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let ttype = self.ttype_of_ident_or_keyword(&text);
        self.add_ident_or_keyword(ttype);
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(cha) = ch {
            return cha.is_ascii_alphanumeric() || cha == '_';
        }
        false
    }

    fn ttype_of_ident_or_keyword(&self, check: &str) -> TokenType {
        if self.keywords.contains_key(check) {
            return self.keywords.get(check).copied().unwrap();
        }
        TokenType::Identifier
    }

    fn add_ident_or_keyword(&mut self, ttype: TokenType) {
        match ttype {
            TokenType::True => {
                self.add_token(ttype, Some(Literal::Bool(true)));
            }
            TokenType::False => {
                self.add_token(ttype, Some(Literal::Bool(false)));
            }
            TokenType::Nil => {
                self.add_token(ttype, Some(Literal::Nil));
            }
            _ => {
                self.add_terminator(ttype);
            }
        }
    }

    fn scan_block_comments(&mut self) -> Result<(), JialoxError> {
        loop {
            match self.currentc() {
                Some('*') => {
                    self.advance();
                    if self.next_match('/') {
                        return Ok(());
                    }
                }
                Some('/') => {
                    self.advance();
                    if self.next_match('*') {
                        self.scan_block_comments()?;
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(JialoxError::error(self.line, "Unterminated block comments"));
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
