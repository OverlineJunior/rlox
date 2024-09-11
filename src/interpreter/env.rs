use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::runtime_error::{undefined_variable, RuntimeError};
use crate::scanner::{literal::Literal, token::Token};

pub struct Env {
    bindings: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<Env>>>,
}

impl Env {
    /// Returns an environment with no parent, aka global.
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            bindings: HashMap::new(),
            enclosing: None,
        }))
    }

    /// Returns an environment with a parent.
    pub fn new_enclosed(enclosing: &Rc<RefCell<Env>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            bindings: HashMap::new(),
            enclosing: Some(Rc::clone(enclosing)),
        }))
    }

    /// Returns the value bound to ´name´ in the current or above scopes.
    /// Errors if binding could not be found.
    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.bindings.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
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
        match self.bindings.entry(name.lexeme.clone()) {
            Entry::Occupied(mut entry) => {
                let old = entry.insert(value);
                Ok(old)
            }
            Entry::Vacant(_) => match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(undefined_variable(name)),
            },
        }
    }
}

mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::Env;
    use crate::scanner::{literal::Literal, token::Token, token_kind::TokenKind as TK};

    #[test]
    fn test() {
        let and = Token::symbol(TK::And, "and".into(), 1);
        let one = Literal::Number(1.0);
        let two = Literal::Number(2.0);
        let three = Literal::Number(3.0);
        let global = Env::new();
        let child = Env::new_enclosed(&global);

        global.borrow_mut().define(and.clone(), one.clone());
        let _ = child.borrow_mut().assign(and.clone(), two.clone());

        assert_eq!(global.borrow().get(and.clone()).unwrap(), two);
    }
}
