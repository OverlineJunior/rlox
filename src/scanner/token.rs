use super::{literal::Literal, token_kind::TokenKind};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal: Some(literal),
            line,
        }
    }

    /// A symbol (or symbolic token) is a token that does not have a literal value.
    pub fn symbol(kind: TokenKind, lexeme: String, line: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(lit) => write!(f, "{}", lit),
            None => write!(f, "{}", self.lexeme),
        }
    }
}
