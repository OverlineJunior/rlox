use crate::cursor::Cursor;

pub const EOF: char = '\0';

pub struct StringCursor {
	cursor: Cursor<char>,
	checkpoint: usize,
	line: usize,
}

impl StringCursor {
	pub fn new(source: String) -> Self {
		Self {
			cursor: Cursor::new(source.chars().collect()),
			checkpoint: 0,
			line: 1,
		}
	}

    /// Returns the character before the one currently being pointed to or EOF if the cursor is at the start of the source.
	pub fn prev(&self) -> char {
		self.cursor.prev().unwrap_or(EOF)
	}

    /// Returns the character currently being pointed to or EOF if the cursor is at the end of the source.
	pub fn current(&self) -> char {
		self.cursor.current().unwrap_or(EOF)
	}

    /// Returns the character after the one currently being pointed to or EOF if the next position is at the end of the source.
	pub fn next(&self) -> char {
		self.cursor.next().unwrap_or(EOF)
	}

    /// Returns the current line number.
	pub fn line(&self) -> usize {
		self.line
	}

    /// Returns true if the cursor is at the end of the source.
	pub fn is_eof(&self) -> bool {
		self.current() == EOF
	}

    /// Sets a checkpoint at the current cursor position.
	pub fn set_checkpoint(&mut self) {
		self.cursor.set_checkpoint()
	}

    /// Returns every char eaten since the last checkpoint.
	pub fn chars_since_checkpoint(&self) -> Vec<char> {
		self.cursor.since_checkpoint()
	}

    /// Returns a `String` composed of every char eaten since the last checkpoint.
	pub fn string_since_checkpoint(&self) -> String {
		self.chars_since_checkpoint().iter().collect()
	}

    /// Eats the current character and returns it. Returns EOF if the cursor is at the end of the source.
	pub fn eat(&mut self) -> char {
		self.cursor.eat().unwrap_or(EOF)
	}

    /// Repeatedly eats characters while the predicate returns true. Returns the characters eaten.
	pub fn eat_while(&mut self, predicate: impl Fn(char) -> bool) -> Vec<char> {
		self.cursor.eat_while(predicate)
	}
}

mod tests {
    use super::StringCursor;

    #[test]
    fn test_cursor() {
        let mut cursor = StringCursor::new("Hello, world!".into());

        for _ in 0..5 {
            cursor.eat();
        }
        assert_eq!(
            cursor.string_since_checkpoint(),
            "Hello".to_string()
        );

        cursor.set_checkpoint();
        cursor.eat();
        assert_eq!(cursor.chars_since_checkpoint().first(), Some(&','));

        assert_eq!(cursor.current(), ' ');
        cursor.eat();

        cursor.set_checkpoint();
        for _ in 0..5 {
            cursor.eat();
        }
        assert_eq!(
            cursor.string_since_checkpoint(),
            "world".to_string()
        );

        assert!(!cursor.is_eof());
        cursor.eat();
        assert!(cursor.is_eof());
    }
}
