use std::collections::HashMap;

use crate::error::*;
use crate::literal::*;
use crate::token::*;


pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Literal, JialoxError> {
        if let Some(literal) = self.values.get(&name.lexeme()) {
            Ok(literal.clone())
        } else {
            Err(JialoxError::runtime_error(
                name,
                &format!("var {} is not defined.", name.lexeme()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::*;

    #[test]
    fn can_define_a_variable() {
        let mut e = Environment::new();
        e.define("one", Literal::Bool(true));
        assert!(e.values.contains_key("one"));
        assert_eq!(e.values.get("one").unwrap(), &Literal::Bool(true));
    }

    #[test]
    fn can_redefine_a_variable() {
        let mut e = Environment::new();
        e.define("one", Literal::Bool(true));
        e.define("one", Literal::Num(12.8));
        assert!(e.values.contains_key("one"));
        assert_eq!(e.values.get("one").unwrap(), &Literal::Num(12.8));
    }

    #[test]
    fn can_look_up_a_variable() {
        let mut e = Environment::new();
        e.define("three", Literal::Str("foo".to_string()));
        let tok = Token::new(TokenType::Identifier, "three".to_string(), None, 1);
        let result = e.get(&tok);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Literal::Str("foo".to_string()));
    }

    #[test]
    fn error_when_variable_undefined() {
        let e = Environment::new();
        let tok = Token::new(TokenType::Identifier, "three".to_string(), None, 1);
        assert!(e.get(&tok).is_err());
    }
}
