use crate::{
    cursor::{Cursor, EOF},
    token::Token,
    token_kind::TokenKind as TK,
};

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t' | '\n')
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
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
            '!' => {
                if self.current() == '=' {
                    self.eat();
                    TK::BangEqual
                } else {
                    TK::Bang
                }
            }

            '=' => {
                if self.current() == '=' {
                    self.eat();
                    TK::EqualEqual
                } else {
                    TK::Equal
                }
            }

            '<' => {
                if self.current() == '=' {
                    self.eat();
                    TK::LessEqual
                } else {
                    TK::Less
                }
            }

            '>' => {
                if self.current() == '=' {
                    self.eat();
                    TK::GreaterEqual
                } else {
                    TK::Greater
                }
            }

            // Multiple lexemes.
            '/' => {
                if self.current() == '/' {
                    self.skip_line_comment();
                    return self.eat_token();
                } else {
                    TK::Slash
                }
            }

            '"' => return self.eat_string_token(),

            c if c.is_ascii_digit() => return self.eat_number_token(),

            c if is_identifier_start(c) => return Ok(self.eat_identifier_token()),

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

    fn eat_string_token(&mut self) -> Result<Option<Token>, String> {
        assert_eq!(
            self.prev(),
            '"',
            "Should be called after eating the opening quote"
        );

        self.eat_while(|c| c != '"' && c != EOF);

        if self.is_eof() {
            return Err("Unterminated string".into());
        }

        // The closing quote.
        self.eat();

        let lexeme = &self.string_since_checkpoint();
        let literal = lexeme.trim_matches('"');
        Ok(Some(Token::new(
            TK::String,
            lexeme.into(),
            literal.into(),
            self.line(),
        )))
    }

    fn eat_number_token(&mut self) -> Result<Option<Token>, String> {
        assert!(
            self.prev().is_ascii_digit(),
            "Should be called after eating the first digit"
        );

        self.eat_while(|c| c.is_ascii_digit());

        if self.current() == '.' {
            if !self.next().is_ascii_digit() {
                return Err("Digit expected after dot".into());
            }

            self.eat();
            self.eat_while(|c| c.is_ascii_digit());
        }

        let lexeme = &self.string_since_checkpoint();
        let literal = lexeme.parse::<f64>().expect("Should be a valid number");
        Ok(Some(Token::new(
            TK::Number,
            lexeme.into(),
            literal.into(),
            self.line(),
        )))
    }

    fn eat_identifier_token(&mut self) -> Option<Token> {
        assert!(
            is_identifier_start(self.prev()),
            "Should be called after eating the first identifier character"
        );

        self.eat_while(is_identifier_continue);

        let lexeme = &self.string_since_checkpoint();
        let kind = TK::keyword_from(lexeme).unwrap_or(TK::Identifier);
        Some(Token::symbol(kind, lexeme.into(), self.line()))
    }

    fn skip_line_comment(&mut self) {
        self.eat_while(|c| c != '\n' && c != EOF);
    }
}

#[cfg(test)]
mod tests {
    use super::tokenize;
    use std::{fs, path::Path};

    #[test]
    fn test_tokenize() {
        let source = fs::read_to_string(Path::new("test_source")).unwrap();
        let tokens = tokenize(source).unwrap();
        println!("{:#?}", tokens);
    }
}
