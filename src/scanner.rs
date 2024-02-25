use crate::token::{Token, TokenKind};

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
}

impl Scanner {
	fn new(source: String) -> Scanner {
		Scanner {
			source,
			tokens: Vec::new(),
			start: 0,
			current: 0,
			line: 1,
		}
	}

	fn scan_source(&mut self) -> Result<Vec<Token>, String> {
		while !self.at_end() {
			self.scan_next_token()?;
		}

		self.tokens.push(Token::new(TokenKind::Eof, String::new(), self.line));
		Ok(self.tokens.clone())
	}

	fn scan_next_token(&mut self) -> Result<(), String> {
		let old_char = self.char_at(self.current)?;
		self.start = self.current;
		self.current += 1;

		match old_char {
			// One lexeme.
			'(' => self.push_token(TokenKind::LeftParenthesis),
			')' => self.push_token(TokenKind::RightParenthesis),
			'{' => self.push_token(TokenKind::LeftBrace),
			'}' => self.push_token(TokenKind::RightBrace),
			',' => self.push_token(TokenKind::Comma),
			'.' => self.push_token(TokenKind::Dot),
			'+' => self.push_token(TokenKind::Plus),
			'-' => self.push_token(TokenKind::Minus),
			';' => self.push_token(TokenKind::Semicolon),
			'*' => self.push_token(TokenKind::Star),

			// Two lexemes.
			'!' => if self.current_char_is('=') {
				self.current += 1;
				self.push_token(TokenKind::BangEqual);
			} else {
				self.push_token(TokenKind::Bang);
			},
			'=' => if self.current_char_is('=') {
				self.current += 1;
				self.push_token(TokenKind::EqualEqual);
			} else {
				self.push_token(TokenKind::Equal);
			},
			'<' => if self.current_char_is('=') {
				self.current += 1;
				self.push_token(TokenKind::LessEqual);
			} else {
				self.push_token(TokenKind::Less);
			},
			'>' => if self.current_char_is('=') {
				self.current += 1;
				self.push_token(TokenKind::GreaterEqual);
			} else {
				self.push_token(TokenKind::Greater);
			},

			// Multiple lexemes.
			'/' => if self.current_char_is('/') {
				// Ignore everything until a newline is found.
				while self.char_at(self.current)? != '\n' && !self.at_end() {
					self.current += 1;
				}
			} else {
				self.push_token(TokenKind::Slash);
			},
			'"' => self.push_string_token()?,

			// Ignore whitespace.
			' ' | '\r' | '\t' => (),

			'\n' => self.line += 1,

			_ => return Err(format!("Unexpected character `{old_char}`")),
		};

		Ok(())
	}

	fn push_token(&mut self, kind: TokenKind) {
		let lexeme = &self.source[self.start..self.current];
		self.tokens.push(Token::new(kind, lexeme.to_string(), self.line));
	}

	fn push_string_token(&mut self) -> Result<(), String> {
		while self.char_at(self.current)? != '"' && !self.at_end() {
			if self.char_at(self.current)? == '\n' {
				self.line += 1;
			}

			self.current += 1;
		}

		if self.at_end() {
			return Err("Unterminated string".to_string());
		}

		let value = self.source.trim_matches('"');
		self.tokens.push(Token::new(TokenKind::String, value.to_string(), self.line));

		Ok(())
	}

	fn current_char_is(&self, subject: char) -> bool {
		self.char_at(self.current).is_ok_and(|c| c == subject)
	}

	fn char_at(&self, index: usize) -> Result<char, String> {
		if let Some(c) = self.source.chars().nth(index) {
			println!("{c}");
		}

		self.source.chars().nth(index).ok_or(format!("Character index `{index}` out of bounds"))
	}

	fn at_end(&self) -> bool {
		self.current >= self.source.len() - 1
	}
}

#[cfg(test)]
mod tests {
	use std::{fs, path::Path};
	use super::Scanner;

	#[test]
	fn foo() {
		let src = fs::read_to_string(Path::new("test_source")).unwrap();
		let mut scanner = Scanner::new(src);
		let tokens = scanner.scan_source().unwrap();
		println!("Tokens found: {} ---- {:#?}", tokens.len(), tokens);
	}
}
