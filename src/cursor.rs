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

    /// Returns the character before the one currently being pointed to. Panics if the cursor is at the start of the source.
    pub fn prev(&self) -> char {
        assert!(
            self.position > 0,
            "Cannot go back from the start of the source"
        );

        self.source
            .chars()
            .nth(self.position - 1)
            .expect("Previous character should never be EOF")
    }

    /// Returns the character currently being pointed to or EOF if the cursor is at the end of the source.
    pub fn current(&self) -> char {
        self.source.chars().nth(self.position).unwrap_or(EOF)
    }

    /// Returns the character after the one currently being pointed to or EOF if the next position is at the end of the source.
    pub fn next(&self) -> char {
        self.source.chars().nth(self.position + 1).unwrap_or(EOF)
    }

    /// Sets a checkpoint at the current cursor position.
    pub fn set_checkpoint(&mut self) {
        self.checkpoint = self.position;
    }

    /// Returns every char eaten since the last checkpoint.
    pub fn chars_since_checkpoint(&self) -> Chars {
        self.source[self.checkpoint..self.position].chars()
    }

    /// Returns a `String` composed of every char eaten since the last checkpoint.
    pub fn string_since_checkpoint(&self) -> String {
        self.chars_since_checkpoint().collect::<String>()
    }

    /// Returns the current line number.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Returns true if the cursor is at the end of the source.
    pub fn is_eof(&self) -> bool {
        self.current() == EOF
    }

    /// Eats the current character and returns it. Returns None if the cursor is at the end of the source.
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

    /// Repeatedly eats characters while the predicate returns true.
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

        for _ in 0..5 {
            cursor.eat();
        }
        assert_eq!(
            cursor.chars_since_checkpoint().collect::<String>(),
            "Hello".to_string()
        );

        cursor.set_checkpoint();
        cursor.eat();
        assert_eq!(cursor.chars_since_checkpoint().next(), Some(','));

        assert_eq!(cursor.current(), ' ');
        cursor.eat();

        cursor.set_checkpoint();
        for _ in 0..5 {
            cursor.eat();
        }
        assert_eq!(
            cursor.chars_since_checkpoint().collect::<String>(),
            "world".to_string()
        );

        assert!(!cursor.is_eof());
        cursor.eat();
        assert!(cursor.is_eof());
    }
}
