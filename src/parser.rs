use std::rc::Rc;

use crate::error::*;
use crate::expr::*;
use crate::literal::Literal;
use crate::token::*;
use crate::token_type::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, JialoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, JialoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().mirror();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, JialoxError> {
        let mut comp = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().mirror();
            let right = self.term()?;
            comp = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(comp),
                operator,
                right: Rc::new(right),
            }));
        }
        Ok(comp)
    }

    fn term(&mut self) -> Result<Expr, JialoxError> {
        let mut te = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().mirror();
            let right = self.factor()?;
            te = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(te),
                operator,
                right: Rc::new(right),
            }));
        }
        Ok(te)
    }

    fn factor(&mut self) -> Result<Expr, JialoxError> {
        let mut fac = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().mirror();
            let right = self.unary()?;
            fac = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(fac),
                operator,
                right: Rc::new(right),
            }));
        }
        Ok(fac)
    }

    fn unary(&mut self) -> Result<Expr, JialoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().mirror();
            let right = self.unary()?;
            return Ok(Expr::Unary(Rc::new(UnaryExpr {
                operator,
                right: Rc::new(right),
            })));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, JialoxError> {
        if self.is_match(&[
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: self.previous().literal(),
            })));
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expected ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }
        Err(JialoxError::error(0, "Expected expression".to_string()))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, JialoxError> {
        if self.check(ttype) {
            Ok(self.advance().mirror())
        } else {
            let tok = self.currentt();
            Err(JialoxError::error(tok.line(), message))
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.currentt().is(ttype)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.currentt().is(TokenType::Eof)
    }

    fn currentt(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}