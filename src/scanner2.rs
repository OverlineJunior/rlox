use crate::{cursor::{Cursor, EOF}, token::Token, token_kind::TokenKind as TK};

fn is_whitespace(c: char) -> bool {
	matches!(c, ' ' | '\r' | '\t' | '\n')
}

pub fn tokenize(source: String) -> Result<Vec<Token>, String> {
	let mut cursor = Cursor::new(source);
	let mut tokens: Vec<Token> = Vec::new();

	loop {
		match cursor.eat_token() {
			Ok(Some(token)) => tokens.push(token),
			Ok(None) => break,
			Err(e) => return Err(e),
		}
	}

	Ok(tokens)
}

impl Cursor {
	/// Attempts to eat the next token.
	/// Returns `None` if at EOF or if no token was found before it
	/// (e.g. trailing whitespace at the end).
	pub fn eat_token(&mut self) -> Result<Option<Token>, String> {
		self.set_checkpoint();

		if self.is_eof() {
			return Ok(None);
		}

		// Symbols are returned later to avoid duplication.
		let symbol_kind = match self.eat().expect("Should not be EOF") {
			// Single lexeme.
			'(' => TK::LeftParenthesis,
			')' => TK::RightParenthesis,
		    '{' => TK::LeftBrace,
			'}' => TK::RightBrace,
			',' => TK::Comma,
			'.' => TK::Dot,
			'+' => TK::Plus,
			'-' => TK::Minus,
			';' => TK::Semicolon,
			'*' => TK::Star,

			// Double lexeme.
			'!' => if self.current() == '=' {
				self.eat();
				TK::BangEqual
			} else {
				TK::Bang
			},

			'=' => if self.current() == '=' {
				self.eat();
				TK::EqualEqual
			} else {
				TK::Equal
			},

			'<' => if self.current() == '=' {
				self.eat();
				TK::LessEqual
			} else {
				TK::Less
			},

			'>' => if self.current() == '=' {
				self.eat();
				TK::GreaterEqual
			} else {
				TK::Greater
			},

			// Multiple lexemes.
			'/' => if self.current() == '/' {
				self.skip_line_comment();
				return self.eat_token();
			} else {
				TK::Slash
			},

			// Ignore whitespace.
			c if is_whitespace(c) => return self.eat_token(),

			c => return Err(format!("Unexpected character `{}`", c)),
		};

		Ok(Some(Token::symbol(
			symbol_kind,
			self.chars_since_checkpoint().collect(),
			self.line(),
		)))
	}

	fn skip_line_comment(&mut self) {
		self.eat_while(|c| c != '\n' && c != EOF);
	}
}

#[cfg(test)]
mod tests {
	use std::{fs, path::Path};
	use super::tokenize;

	#[test]
	fn test_tokenize() {
		let source = fs::read_to_string(Path::new("test_source_2")).unwrap();
		let tokens = tokenize(source).unwrap();
		println!("{:#?}", tokens);
	}
}
