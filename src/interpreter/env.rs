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

	pub fn get(&self, name: String) -> Option<&Literal> {
		self.bindings.get(&name)
	}

	/// Returns the overwritten `Literal` if there was one.
	/// If `value` is `None`, it is defaulted to `Literal::Nil`.
	pub fn set(&mut self, name: String, value: Option<Literal>) -> Option<Literal> {
		self.bindings.insert(name, value.unwrap_or(Literal::Nil))
	}
}
