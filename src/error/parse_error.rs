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
    EmptyExpression,
    NotParseable {
        token: TokenKind,
        line: usize,
    },
    ExpectedSemicolon {
        got: Option<TokenKind>,
        line: usize,
    },
    BadAssignmentTarget {
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
                    write!(f, "[line {line}] Expected `{:?}`, got `{:?}`", expected, got)
                } else {
                    write!(f, "[line {line}] Expected `{:?}`", expected)
                }
            }
            ParseError::ExpectedAnyToken { line } => {
                write!(f, "[line {line}] Expected token")
            }
            ParseError::ExpectedAnyLeftOperand { operator, line } => {
                write!(f, "[line {line}] Expected left operand for `{:?}`", operator)
            }
            ParseError::EmptyExpression => {
                write!(f, "Expression cannot be empty")
            }
            ParseError::NotParseable { token, line } => {
                write!(f, "[line {line}] `{:?}` cannot be turned into an expression", token)
            }
            ParseError::ExpectedSemicolon { got, line } => {
                if let Some(got) = got {
                    write!(f, "[line {line}] Expected `;`, got `{:?}`", got)
                } else {
                    write!(f, "[line {line}] Expected `;`")
                }
            }
            ParseError::BadAssignmentTarget { line } => {
                write!(f, "[line {line}] Invalid assignment target")
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
