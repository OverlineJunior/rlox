use std::collections::{hash_map::Entry, HashMap};

use crate::{
    error::runtime_error::{undefined_variable, RuntimeError},
    literal::Literal,
    token::Token,
};

#[derive(Clone)]
pub struct Env {
    bindings: HashMap<String, Literal>,
    enclosing: Option<Box<Env>>,
}

impl Env {
    /// Returns an environment with no parent, aka global.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            enclosing: None,
        }
    }

    /// Returns an environment with a parent.
    pub fn new_enclosed(enclosing: Env) -> Self {
        Self {
            bindings: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    /// Returns the value bound to ´name´ in the current or above scopes.
    /// Errors if binding could not be found.
    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.bindings.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            Err(undefined_variable(name))
        }
    }

    /// Defines a new binding or overwrites the old one, returning it.
    pub fn define(&mut self, name: Token, value: Literal) -> Option<Literal> {
        self.bindings.insert(name.lexeme, value)
    }

    /// Assigns a value to an already existing binding in the current or above scopes,
    /// returning the old value.
    /// Errors if binding could not be found.
    pub fn assign(&mut self, name: Token, value: Literal) -> Result<Literal, RuntimeError> {
        #[allow(clippy::map_entry)]
        if self.bindings.contains_key(&name.lexeme) {
            let old = self
                .bindings
                .insert(name.lexeme, value)
                .expect("Should have the key");

            return Ok(old);
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }

        Err(undefined_variable(name))
    }
}

mod tests {
    use crate::{literal::Literal, token::Token, token_kind::TokenKind};
    use super::Env;

    #[test]
    fn test() {
        let and = Token::symbol(TokenKind::And, "and".into(), 1);
        let one = Literal::Number(1.0);
        let two = Literal::Number(2.0);
        let three = Literal::Number(3.0);

        let mut global = Env::new();
        global.define(and.clone(), one.clone());

        let mut child = Env::new_enclosed(global.clone());
        let _ = child.assign(and.clone(), two.clone());

        global.define(and.clone(), three.clone());

        assert!(global.get(and.clone()).is_ok_and(|l| l == three));
        assert!(child.get(and).is_ok_and(|l| l == two));
    }
}
