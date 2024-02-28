use std::str::Chars;

pub const EOF: char = '\0';

pub struct Cursor {
	source: String,
	position: usize,
	checkpoint: usize,
	line: usize,
}

impl Cursor {
	pub fn new(source: String) -> Cursor {
		Cursor {
			source,
			position: 0,
			checkpoint: 0,
			line: 1,
		}
	}

	pub fn prev(&self) -> char {
		assert!(self.position > 0, "Cannot go back from the start of the source");

		self.source.chars().nth(self.position - 1).expect("Previous character should never be EOF")
	}

	pub fn current(&self) -> char {
		self.source.chars().nth(self.position).unwrap_or(EOF)
	}

	pub fn next(&self) -> char {
		self.source.chars().nth(self.position + 1).unwrap_or(EOF)
	}

	pub fn set_checkpoint(&mut self) {
		self.checkpoint = self.position;
	}

	pub fn chars_since_checkpoint(&self) -> Chars {
		self.source[self.checkpoint..self.position].chars()
	}

	pub fn line(&self) -> usize {
		self.line
	}

	pub fn is_eof(&self) -> bool {
		self.current() == EOF
	}

	pub fn eat(&mut self) -> Option<char> {
		let c = self.current();

		match c {
			EOF => return None,
			'\n' => self.line += 1,
			_ => (),

		};

		self.position += 1;
		Some(c)
	}

	pub fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
		while !self.is_eof() && predicate(self.current()) {
			self.eat();
		}
	}
}

mod tests {
	use super::Cursor;

	#[test]
	fn test_cursor() {
		let mut cursor = Cursor::new("Hello, world!".into());

		for _ in 0..5 { cursor.eat(); }
		assert_eq!(cursor.chars_since_checkpoint().collect::<String>(), "Hello".to_string());

		cursor.set_checkpoint();
		cursor.eat();
		assert_eq!(cursor.chars_since_checkpoint().next(), Some(','));

		assert_eq!(cursor.current(), ' ');
		cursor.eat();

		cursor.set_checkpoint();
		for _ in 0..5 { cursor.eat(); }
		assert_eq!(cursor.chars_since_checkpoint().collect::<String>(), "world".to_string());

		assert!(!cursor.is_eof());
		cursor.eat();
		assert!(cursor.is_eof());
	}
}
