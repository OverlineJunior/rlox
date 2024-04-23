use crate::{
    error::runtime_error::{bad_bin_ops, bad_un_op, div_by_zero, undefined_variable, RuntimeError},
    expr::Expr,
    literal::Literal,
    token_kind::TokenKind as TK,
};

use super::env::Env;

/// Evaluates a single expression tree and returns the resulting literal.
/// This is the expression analogue of `execute`.
pub fn eval(expr: Expr, env: &mut Env) -> Result<Literal, RuntimeError> {
    match expr {
        Expr::Literal(literal) => Ok(literal.clone()),

        Expr::Unary(op, r) => {
            let r = eval(*r, env)?;

            match op.kind {
                TK::Minus => match r {
                    Literal::Number(n) => Ok(Literal::Number(-n)),

                    _ => Err(bad_un_op(op.kind, r, op.line)),
                },

                _ => panic!("Invalid unary operator `{:?}`", op.kind),
            }
        }

        Expr::Binary(l, op, r) => {
            let l = eval(*l, env)?;
            let r = eval(*r, env)?;

            match op.kind {
                TK::Plus => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),

                    // "foo" + "bar", "foo" + 1, ...
                    (Literal::String(l), r) => Ok(Literal::String(format!("{}{}", l, r))),

                    // "foo" + "bar", 1 + "bar", ...
                    (l, Literal::String(r)) => Ok(Literal::String(format!("{}{}", l, r))),

                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::Minus => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l - r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::Star => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l * r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::Slash => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => {
                        if r == &0. {
                            return Err(div_by_zero(Literal::Number(*l), op.line));
                        }

                        Ok(Literal::Number(l / r))
                    }

                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::Greater => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l > r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::GreaterEqual => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l >= r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::Less => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l < r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::LessEqual => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l <= r)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::EqualEqual => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l == r)),
                    (Literal::String(l), Literal::String(r)) => Ok(Literal::Bool(l == r)),
                    (Literal::Bool(l), Literal::Bool(r)) => Ok(Literal::Bool(l == r)),
                    (Literal::Nil, Literal::Nil) => Ok(Literal::Bool(true)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                TK::BangEqual => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Bool(l != r)),
                    (Literal::String(l), Literal::String(r)) => Ok(Literal::Bool(l != r)),
                    (Literal::Bool(l), Literal::Bool(r)) => Ok(Literal::Bool(l != r)),
                    (Literal::Nil, Literal::Nil) => Ok(Literal::Bool(false)),
                    _ => Err(bad_bin_ops(op.kind, l, r, op.line)),
                },

                _ => panic!("Invalid binary operator `{:?}`", op.kind),
            }
        }

        Expr::Group(expr) => eval(*expr, env),

        Expr::Ternary(expr, if_, else_) => {
            let cond = eval(*expr, env)?;

            if cond.is_truthy() {
                eval(*if_, env)
            } else {
                eval(*else_, env)
            }
        }

        Expr::Variable { name } => {
            env.get(&name.lexeme).ok_or(undefined_variable(name))
        },
    }
}
