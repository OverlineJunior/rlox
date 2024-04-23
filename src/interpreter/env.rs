use std::collections::HashMap;

use crate::{literal::Literal, token::Token};

pub struct Env {
	bindings: HashMap<String, Literal>,
}

impl Env {
	fn new() -> Self {
		Self {
			bindings: HashMap::new(),
		}
	}

	fn get(&self, name: String) -> Option<&Literal> {
		self.bindings.get(&name)
	}

	/// Returns the overwritten literal if there was one.
	fn set(&mut self, name: String, value: Literal) -> Option<Literal> {
		self.bindings.insert(name, value)
	}
}
