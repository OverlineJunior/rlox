use std::collections::HashMap;

use crate::{literal::Literal, token::Token};

pub struct Env {
    bindings: HashMap<String, Literal>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Literal> {
        self.bindings.get(name).cloned()
    }

    /// Defines a new binding or overwrites the old one, returning it.
    pub fn define(&mut self, name: &str, value: Literal) -> Option<Literal> {
        self.bindings.insert(name.into(), value)
    }

    /// Assigns a value to an already existing binding, returning the old value.
    /// Errors if said binding does not exist.
    pub fn assign(&mut self, name: &str, value: Literal) -> Result<Literal, ()> {
        if !self.bindings.contains_key(name) {
            return Err(());
        }

        let old_value = self
            .bindings
            .insert(name.into(), value)
            .expect("Should have the key");

        Ok(old_value)
    }
}
