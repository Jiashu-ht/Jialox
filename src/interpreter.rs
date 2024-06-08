use std::rc::Rc;

use crate::expr::*;
use crate::literal::*;
use crate::error::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Literal, JialoxError> {
        
        Ok(Literal::Nil)
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Literal, JialoxError> {
        self.evaluate(expr.expression.clone())
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Literal, JialoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Literal, JialoxError> {
        let right = self.evaluate(expr.right.clone())?;
        
        match expr.operator.ttype() {
            TokenType::Minus => match right {
                Literal::Num(val) => return Ok(Literal::Num(-val)),
                _ => return Ok(Literal::Nil),
            },
            TokenType::Bang => {
                if self.is_truthy(&right) {
                    Ok(Literal::Bool(false))
                } else {
                    Ok(Literal::Bool(true))
                }
            }
            _ => Err(JialoxError::error(
                0, 
                "Unreachable in struct Interpreter's method visit_unary_expr()"
            ))
        } 
    }
}

impl Interpreter {
    fn evaluate(&self, expr: Rc<Expr>) -> Result<Literal, JialoxError> {
        expr.accept(self)
    }

    /// false and nil are falsey, and everything else is truthy.
    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Bool(false) | Literal::Nil)
    }
}
