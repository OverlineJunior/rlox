use crate::token_kind::TokenKind;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum ParseError {
    ExpectedToken {
        expected: TokenKind,
        got: Option<TokenKind>,
        line: usize,
    },
    ExpectedAnyToken {
        line: usize,
    },
    ExpectedAnyLeftOperand {
        operator: TokenKind,
        line: usize,
    },
    NotParseable {
        token: TokenKind,
        line: usize,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ExpectedToken {
                expected,
                got,
                line,
            } => {
                if let Some(got) = got {
                    write!(f, "Expected `{:?}`, got `{:?}`", expected, got)
                } else {
                    write!(f, "Expected `{:?}`", expected)
                }
            }
            ParseError::ExpectedAnyToken { line } => {
                write!(f, "Expected token")
            }
            ParseError::ExpectedAnyLeftOperand { operator, line } => {
                write!(f, "Expected left operand for `{:?}`", operator)
            }
            ParseError::NotParseable { token, line } => {
                write!(f, "`{:?}` cannot be turned into an expression", token)
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
