pub mod literal;
pub mod scan_error;
pub mod token;
pub mod token_kind;

use self::{
    literal::Literal,
    scan_error::ScanError::{self, *},
    token::Token,
    token_kind::TokenKind as TK,
};
use crate::cursor::string_cursor::{StringCursor, EOF};

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t' | '\n')
}

fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

pub fn tokenize(source: String) -> Result<Vec<Token>, ScanError> {
    let mut cursor = StringCursor::new(source);
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

impl StringCursor {
    /// Attempts to eat the next token.
    /// Returns `None` if at EOF or if no token was found before it
    /// (e.g. trailing whitespace at the end).
    pub fn eat_token(&mut self) -> Result<Option<Token>, ScanError> {
        self.set_checkpoint();

        if self.is_eof() {
            return Ok(None);
        }

        // Symbols are returned later to avoid duplication.
        let symbol_kind = match self.eat() {
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
            '?' => TK::Question,
            ':' => TK::Colon,

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

            c => {
                return Err(UnexpectedChar {
                    ch: c,
                    line: self.line(),
                })
            }
        };

        Ok(Some(Token::symbol(
            symbol_kind,
            self.string_since_checkpoint(),
            self.line(),
        )))
    }

    fn eat_string_token(&mut self) -> Result<Option<Token>, ScanError> {
        assert_eq!(
            self.prev(),
            '"',
            "Should be called after eating the opening quote"
        );

        self.eat_while(|c| c != '"' && c != EOF);

        if self.is_eof() {
            return Err(UnterminatedString { line: self.line() });
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

    fn eat_number_token(&mut self) -> Result<Option<Token>, ScanError> {
        assert!(
            self.prev().is_ascii_digit(),
            "Should be called after eating the first digit"
        );

        self.eat_while(|c| c.is_ascii_digit());

        if self.current() == '.' {
            if !self.next().is_ascii_digit() {
                return Err(ExpectedDigitAfterDot { line: self.line() });
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

        let lit = match kind {
            TK::True => Some(Literal::Bool(true)),
            TK::False => Some(Literal::Bool(false)),
            TK::Nil => Some(Literal::Nil),
            _ => None,
        };

        if let Some(l) = lit {
            Some(Token::new(kind, lexeme.into(), l, self.line()))
        } else {
            Some(Token::symbol(kind, lexeme.into(), self.line()))
        }
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
