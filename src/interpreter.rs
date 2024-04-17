use crate::{
    error::runtime_error::{bad_bin_ops, bad_un_op, RuntimeError},
    expr::Expr,
    literal::Literal,
    token_kind::TokenKind as TK,
};

/// Evaluates a single expression tree and returns the resulting literal.
pub fn interpret(expr: Expr) -> Result<Literal, RuntimeError> {
    match expr {
        Expr::Literal(literal) => Ok(literal.clone()),

        Expr::Unary(op, r) => {
            let r = interpret(*r)?;

            match op.kind {
                TK::Minus => match r {
                    Literal::Number(n) => Ok(Literal::Number(-n)),

                    _ => Err(bad_un_op(op.kind, r, op.line)),
                },

                _ => panic!("Invalid unary operator `{:?}`", op.kind),
            }
        }

        Expr::Binary(l, op, r) => {
            let l = interpret(*l)?;
            let r = interpret(*r)?;

            match op.kind {
                TK::Plus => match (&l, &r) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),

                    (Literal::String(l), Literal::String(r)) => {
                        Ok(Literal::String(format!("{}{}", l, r)))
                    }

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
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l / r)),
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

        Expr::Group(expr) => interpret(*expr),

        Expr::Ternary(expr, if_, else_) => {
            let cond = interpret(*expr)?;

            if cond.is_truthy() {
                interpret(*if_)
            } else {
                interpret(*else_)
            }
        }
    }
}
