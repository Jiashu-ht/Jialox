use std::rc::Rc;

use crate::error::*;
use crate::expr::*;
use crate::literal::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Literal, JialoxError> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;
        let op = expr.operator.ttype();

        let result = match (left, right) {
            (Literal::Num(left), Literal::Num(right)) => match op {
                TokenType::Plus => Literal::Num(left + right),
                TokenType::Minus => Literal::Num(left - right),
                TokenType::Star => Literal::Num(left * right),
                TokenType::Slash => Literal::Num(left / right),
                TokenType::Greater => Literal::Bool(left > right),
                TokenType::GreaterEqual => Literal::Bool(left >= right),
                TokenType::Less => Literal::Bool(left < right),
                TokenType::LessEqual => Literal::Bool(left <= right),
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::EqualEqual => Literal::Bool(left == right),
                _ => Literal::ArithmeticError,
            }
            (Literal::Str(left), Literal::Str(right)) => match op {
                TokenType::Plus => Literal::Str(format!("{left}{right}")),
                TokenType::EqualEqual => Literal::Bool(left == right),
                TokenType::BangEqual => Literal::Bool(left != right),
                _ => Literal::ArithmeticError
            }
            (Literal::Num(left), Literal::Str(right)) => match op {
                TokenType::Plus => Literal::Str(format!("{left}{right}")),
                TokenType::EqualEqual => Literal::Bool(false),
                TokenType::BangEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError
            }
            (Literal::Str(left), Literal::Num(right)) => match op {
                TokenType::Plus => Literal::Str(format!("{left}{right}")),
                TokenType::EqualEqual => Literal::Bool(false),
                TokenType::BangEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError
            }
            (Literal::Bool(left), Literal::Bool(right)) => match op {
                TokenType::EqualEqual => Literal::Bool(left == right),
                TokenType::BangEqual => Literal::Bool(left != right),
                _ => Literal::ArithmeticError
            }
            (Literal::Nil, Literal::Nil) => match op {
                TokenType::EqualEqual => Literal::Bool(true),
                TokenType::BangEqual => Literal::Bool(false),
                _ => Literal::ArithmeticError
            }
            (Literal::Nil, _) => match op {
                TokenType::EqualEqual => Literal::Bool(false),
                TokenType::BangEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError
            }
            _ => Literal::ArithmeticError,
        };
        if result == Literal::ArithmeticError {
            Err(JialoxError::error(
                expr.operator.line(),
                "Illegal expression",
            ))
        } else {
            Ok(result)
        }
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
                Literal::Num(val) => Ok(Literal::Num(-val)),
                _ => Ok(Literal::Nil),
            },
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right))),
            _ => Err(JialoxError::error(
                0,
                "Unreachable in struct Interpreter's method visit_unary_expr()",
            )),
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { }
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<Literal, JialoxError> {
        expr.accept(self)
    }

    /// false and nil are falsey, and everything else is truthy.
    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Bool(false) | Literal::Nil)
    }

    pub fn interpret(&self, expr: Rc<Expr>) -> bool {
        match self.evaluate(expr) {
            Ok(val) => { println!("{val}"); false }
            Err(e) => { e.report(""); true }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn new_literal_number(n: f64) -> Rc<Expr> {
        Rc::new(Expr::Literal(Rc::new(LiteralExpr {
            value: Some(Literal::Num(n)),
        })))
    }

    fn new_literal_boolean(b: bool) -> Rc<Expr> {
        Rc::new(Expr::Literal(Rc::new(LiteralExpr {
            value: Some(Literal::Bool(b)),
        })))
    }

    fn new_literal_str(s: &str) -> Rc<Expr> {
        Rc::new(Expr::Literal(Rc::new(LiteralExpr {
            value: Some(Literal::Str(s.to_string())),
        })))
    }

    fn new_literal_nir() -> Rc<Expr> {
        Rc::new(Expr::Literal(Rc::new(LiteralExpr {
            value: Some(Literal::Nil),
        })))
    }

    fn run_comparison_tests(tok: &Token, cmps_result: Vec<bool>) {
        let nums = vec![7.7, 7.8, 7.9];
        let terp = Interpreter {};

        for (&num, ret) in nums.iter().zip(cmps_result) {
            let binary_expr = BinaryExpr {
                left: new_literal_number(num),
                operator: tok.mirror(),
                right: new_literal_number(7.8),
            };
            let result = terp.visit_binary_expr(&binary_expr);
            assert!(result.is_ok());
            assert_eq!(result.ok(), Some(Literal::Bool(ret)));
        } 
    }

    #[test]
    fn test_unary_minus() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: new_literal_number(57.8),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Num(-57.8)));
    }

    #[test]
    fn test_unary_bang() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 123),
            right: new_literal_boolean(false),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_addition() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_number(7.8),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: new_literal_number(2.0),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Num(9.8)));
    }

    #[test]
    fn test_string_concatination() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_str("hello"),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: new_literal_str(" addition"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(Literal::Str("hello addition".to_string()))
        );
    }

    #[test]
    fn test_substraction() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_number(7.8),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: new_literal_number(4.6),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Num(3.2)));
    }

    #[test]
    fn test_multiplication() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_number(7.8),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 123),
            right: new_literal_number(2.0),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Num(15.6)));
    }

    #[test]
    fn test_division() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_number(7.8),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 123),
            right: new_literal_number(2.0),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Num(3.9)));
    }

    #[test]
    fn test_arithmetic_error_for_substraction() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_number(7.8),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: new_literal_boolean(true),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_greater() {
        run_comparison_tests(
            &Token::new(TokenType::Greater, ">".to_string(), None, 1),
            vec![false, false, true]
        );
    }


    #[test]
    fn test_greatereuqal() {
        run_comparison_tests(
            &Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 1),
            vec![false, true, true]
        );
    }


    #[test]
    fn test_less() {
        run_comparison_tests(
            &Token::new(TokenType::Less, "<".to_string(), None, 1),
            vec![true, false, false]
        );
    }

    #[test]
    fn test_lesseuqal_real_greater() {
        run_comparison_tests(
            &Token::new(TokenType::LessEqual, "<=".to_string(), None, 1),
            vec![true, true, false]
        );
    }


    #[test]
    fn test_equaleuqal_number() {
        run_comparison_tests(
            &Token::new(TokenType::EqualEqual, "==".to_string(), None, 1),
            vec![false, true, false]
        );
    }

    #[test]
    fn test_equaleuqal_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_str("hello"),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: new_literal_str("hello"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_equaleuqal_bool() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_boolean(true),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: new_literal_boolean(true),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_equaleuqal_nil() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_nir(),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: new_literal_nir(),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_bangeuqal_number() {
        run_comparison_tests(
            &Token::new(TokenType::BangEqual, "!=".to_string(), None, 1),
            vec![true, false, true]
        );
    }

    #[test]
    fn test_bangeuqal_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_str("hello"),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            right: new_literal_str("hellx"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_bangeuqal_bool() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_boolean(true),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            right: new_literal_boolean(false),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_bangeuqal_nil() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_nir(),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            right: new_literal_nir(),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(false)));
    }

    #[test]
    fn test_bangeuqal_random() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: new_literal_nir(),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            right: new_literal_number(64.0),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }
}
