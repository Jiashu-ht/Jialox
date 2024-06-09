use std::fmt;

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
