use std::fmt;

use crate::{literal::Literal, token_kind::TokenKind};

#[derive(Clone)]
pub enum RuntimeError {
    InvalidOperandTypes {
        left: Literal,
        operator: TokenKind,
        right: Literal,
        line: usize,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::InvalidOperandTypes {
                left,
                operator,
                right,
                line,
            } => {
                write!(
                    f,
                    "[line {line}] Invalid operands for `{:?}`: `{:?}` and `{:?}`",
                    operator, left, right
                )
            }
        }
    }
}
