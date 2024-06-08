use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::cmp::{PartialOrd, Ordering};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Num(x) => write!(f, "{x}"),
            Literal::Str(x) => write!(f, "{x}"),
            Literal::Bool(x) => write!(f, "{x}"),
            Literal::Nil => write!(f, "nil"),
            Literal::ArithmeticError => panic!("Should not be trying to print this"),
        }
    }
}

impl Add for Literal {
    type Output = Literal;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Num(left), Literal::Num(right)) => Literal::Num(left + right),
            (Literal::Str(left), Literal::Str(right)) => Literal::Str(format!("{left}{right}")),
            _ => Literal::ArithmeticError,
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Num(left), Literal::Num(right)) => Literal::Num(left - right),
            _ => Literal::ArithmeticError,
        }
    }
}

impl Mul for Literal {
    type Output = Literal;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Num(left), Literal::Num(right)) => Literal::Num(left * right),
            _ => Literal::ArithmeticError,
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Num(left), Literal::Num(right)) => Literal::Num(left / right),
            _ => Literal::ArithmeticError,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Literal::Num(left), Literal::Num(right)) => left.partial_cmp(right),
            (Literal::Nil, Literal::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}