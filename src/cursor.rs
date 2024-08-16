use crate::error::parse_error::ParseError::{self, *};

use crate::token::Token;
use crate::token_kind::TokenKind;

pub struct Cursor<T: Clone> {
    source: Vec<T>,
    position: usize,
    checkpoint: usize,
}

impl<T: Clone> Cursor<T> {
    pub fn new(source: Vec<T>) -> Cursor<T> {
        Cursor {
            source,
            position: 0,
            checkpoint: 0,
        }
    }

    /// Returns the value before the one currently being pointed at or None if the cursor is at the start of the source.
    pub fn prev(&self) -> Option<T> {
        if self.position == 0 {
            return None;
        }

        self.source.get(self.position - 1).cloned()
    }

    /// Returns the value currently being pointed or None if the cursor is at the end of the source.
    pub fn current(&self) -> Option<T> {
        self.source.get(self.position).cloned()
    }

    /// Returns the value after the one currently being pointed at or None if the cursor is at the end of the source.
    pub fn next(&self) -> Option<T> {
        self.source.get(self.position + 1).cloned()
    }

    /// Returns true if the cursor is at the end of the source.
    pub fn is_done(&self) -> bool {
        self.source.get(self.position).is_none()
    }

    /// Sets a checkpoint at the current cursor position.
    pub fn set_checkpoint(&mut self) {
        self.checkpoint = self.position;
    }

    /// Returns every value since the last checkpoint.
    pub fn since_checkpoint(&self) -> Vec<T> {
        self.source[self.checkpoint..self.position].to_vec()
    }

    /// Eats the current value and returns it. Returns None if the cursor is that the end of the source.
    pub fn eat(&mut self) -> Option<T> {
        let c = self.current()?;
        self.position += 1;
        Some(c)
    }

    /// Repeatedly eats values while the predicate returns true. Returns the values eaten.
    pub fn eat_while(&mut self, predicate: impl Fn(T) -> bool) -> Vec<T> {
        let mut eaten = Vec::new();

        while !self.is_done() && predicate(self.current().expect("Should be Some")) {
            eaten.push(self.eat().expect("Should be Some"));
        }

        eaten
    }
}

impl Cursor<Token> {
	/// Eats the current token if it is of the specified kind, returning it.
	/// Otherwise, returns an error.
    pub fn eat_kind(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let current = self.current().ok_or(ExpectedToken {
            expected: kind,
            got: None,
            line: self.prev().map(|t| t.line).unwrap_or(0),
        })?;

        if current.kind == kind {
            Ok(self.eat().expect("Should be Some"))
        } else {
            Err(ExpectedToken {
                expected: kind,
                got: Some(current.kind),
                line: 0,
            })
        }
    }
}
