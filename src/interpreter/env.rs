use std::collections::HashMap;

use crate::{
    error::runtime_error::{undefined_variable, RuntimeError},
    literal::Literal,
    token::Token,
};

pub struct Env {
    bindings: HashMap<String, Literal>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Returns the value bound to ´name´.
    /// Errors if binding does not exist.
    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        self.bindings
            .get(&name.lexeme)
            .ok_or(undefined_variable(name))
            .cloned()
    }

    /// Defines a new binding or overwrites the old one, returning it.
    pub fn define(&mut self, name: Token, value: Literal) -> Option<Literal> {
        self.bindings.insert(name.lexeme, value)
    }

    /// Assigns a value to an already existing binding, returning the old value.
    /// Errors if said binding does not exist.
    pub fn assign(&mut self, name: Token, value: Literal) -> Result<Literal, RuntimeError> {
        if !self.bindings.contains_key(&name.lexeme) {
            return Err(undefined_variable(name));
        }

        let old_value = self
            .bindings
            .insert(name.lexeme, value)
            .expect("Should have the key");

        Ok(old_value)
    }
}
