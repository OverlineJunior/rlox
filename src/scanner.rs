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
		self.start = self.current;

		match self.advance().expect("Should not be ran when at end") {
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
			'!' => if self.current_char() == '=' {
				self.advance();
				self.push_token(TokenKind::BangEqual);
			} else {
				self.push_token(TokenKind::Bang);
			},
			'=' => if self.current_char() == '=' {
				self.advance();
				self.push_token(TokenKind::EqualEqual);
			} else {
				self.push_token(TokenKind::Equal);
			},
			'<' => if self.current_char() == '=' {
				self.advance();
				self.push_token(TokenKind::LessEqual);
			} else {
				self.push_token(TokenKind::Less);
			},
			'>' => if self.current_char() == '=' {
				self.advance();
				self.push_token(TokenKind::GreaterEqual);
			} else {
				self.push_token(TokenKind::Greater);
			},

			// Multiple lexemes.
			'/' => if self.current_char() == '/' {
				// Ignore everything until a newline is found.
				while self.current_char() != '\n' && !self.at_end() {
					self.advance();
				}
			} else {
				self.push_token(TokenKind::Slash);
			},
			'"' => self.push_string_token()?,

			// Ignore whitespace.
			' ' | '\r' | '\t' => (),

			'\n' => self.line += 1,

			c => return Err(format!("Unexpected character `{c}`")),
		};

		Ok(())
	}

	fn push_token(&mut self, kind: TokenKind) {
		let lexeme = &self.source[self.start..self.current];
		self.tokens.push(Token::new(kind, lexeme.to_string(), self.line));
	}

	fn push_string_token(&mut self) -> Result<(), String> {
		while self.current_char() != '"' && !self.at_end() {
			if self.current_char() == '\n' {
				self.line += 1;
			}

			self.advance();
		}

		if self.at_end() {
			return Err("Unterminated string".to_string());
		}

		let value = self.source.trim_matches('"');
		self.tokens.push(Token::new(TokenKind::String, value.to_string(), self.line));

		Ok(())
	}

	// Advances to the next character. If the advance was successful, return the old character.
	fn advance(&mut self) -> Option<char> {
		if self.at_end() {
			return None;
		}

		self.current += 1;
		Some(self.char_at(self.current - 1))
	}

	fn current_char(&self) -> char {
		self.char_at(self.current)
	}

	fn char_at(&self, index: usize) -> char {
		self.source.chars().nth(index).expect(&format!("Character index `{index}` out of bounds"))
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
