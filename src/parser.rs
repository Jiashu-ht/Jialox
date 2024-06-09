use std::rc::Rc;

use crate::error::*;
use crate::expr::*;
use crate::stmt::*;
use crate::token::*;
use crate::token_type::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, JialoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(Rc::new(self.declaration()?))
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, JialoxError> {
        let result = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        if result.is_err() {
            self.synchronize();
        }
        result
    }

    fn statement(&mut self) -> Result<Stmt, JialoxError> {
        if self.is_match(&[TokenType::Print]) {
            return Ok(self.print_statement()?);
        }
        Ok(self.expression_statement()?)
    }

    fn print_statement(&mut self) -> Result<Stmt, JialoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print(Rc::new(PrintStmt {
            expression: Rc::new(value),
        })))
    }

    fn expression_statement(&mut self) -> Result<Stmt, JialoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Expression(Rc::new(ExpressionStmt {
            expression: Rc::new(value),
        })))
    }

    fn var_declaration(&mut self) -> Result<Stmt, JialoxError> {
        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;
        let initializer = if self.is_match(&[TokenType::Equal]) {
            Some(Rc::new(self.expression()?))
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration.")?;
        Ok(Stmt::Var(Rc::new(VarStmt { name, initializer })))
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
        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Rc::new(VariableExpr { 
                name: self.previous().mirror() 
            })));
        }
        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }
        Err(JialoxError::error(0, "Expected expression"))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, JialoxError> {
        if self.check(ttype) {
            Ok(self.advance().mirror())
        } else {
            Err(Parser::error(self.currentt(), message))
        }
    }

    fn error(token: &Token, message: &str) -> JialoxError {
        JialoxError::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }
            if matches!(
                self.currentt().ttype(),
                TokenType::Class
                    | TokenType::Func
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
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
        self.previous()
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
