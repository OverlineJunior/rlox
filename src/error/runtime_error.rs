use std::fmt;

use crate::{literal::Literal, token::Token, token_kind::TokenKind};

#[derive(Clone, Debug)]
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
    DivByZero {
        left: Literal,
        line: usize,
    },
    UndefinedVariable {
        name: Token,
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

pub fn div_by_zero(left: Literal, line: usize) -> RuntimeError {
    RuntimeError::DivByZero { left, line }
}

pub fn undefined_variable(name: Token) -> RuntimeError {
    RuntimeError::UndefinedVariable { name }
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

            RuntimeError::DivByZero { left, line } => {
                write!(f, "[line {line}] Cannot divide `{:?}` by zero", left)
            }

            RuntimeError::UndefinedVariable { name } => {
                write!(f, "[line {}] Undefined variable `{}`", name.line, name.lexeme)
            }
        }
    }
}
