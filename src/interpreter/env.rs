use std::collections::HashMap;

use crate::{
    error::runtime_error::{undefined_variable, RuntimeError},
    literal::Literal,
    token::Token,
};

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

    /// Returns the value bound to ´name´ in the current and above scopes.
    /// Errors if binding could not be found..
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

    // TODO! Page 128.
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
