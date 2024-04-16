use std::fmt;

use crate::{literal::Literal, token_kind::TokenKind};

#[derive(Clone)]
pub enum RuntimeError {
    BadUnOp {
        operator: TokenKind,
        right: Literal,
        line: usize,
    },
    BadBinOps {
        left: Literal,
        operator: TokenKind,
        right: Literal,
        line: usize,
    },
}

pub fn bad_un_op(operator: TokenKind, right: Literal, line: usize) -> RuntimeError {
    RuntimeError::BadUnOp {
        operator,
        right,
        line,
    }
}

pub fn bad_bin_ops(
    operator: TokenKind,
    left: Literal,
    right: Literal,
    line: usize,
) -> RuntimeError {
    RuntimeError::BadBinOps {
        left,
        operator,
        right,
        line,
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::BadUnOp {
                operator,
                right,
                line,
            } => {
                write!(
                    f,
                    "[line {line}] Invalid operand for `{:?}`: `{:?}`",
                    operator, right
                )
            }

            RuntimeError::BadBinOps {
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
