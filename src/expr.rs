use crate::{
    error::runtime_error::{bad_bin_ops, bad_un_op, RuntimeError},
    literal::Literal,
    token::Token,
    token_kind::TokenKind as TK,
};

pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group(Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Result<Literal, RuntimeError> {
        match self {
            Expr::Literal(literal) => Ok(literal.clone()),

            Expr::Unary(op, r) => {
                let r = r.eval()?;

                match op.kind {
                    TK::Minus => match r {
                        Literal::Number(n) => Ok(Literal::Number(-n)),

                        _ => Err(bad_un_op(op.kind, r, op.line)),
                    },

                    _ => panic!("Invalid unary operator `{:?}`", op.kind),
                }
            }

            Expr::Binary(l, op, r) => {
                let l = l.eval()?;
                let r = r.eval()?;

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

            Expr::Group(expr) => expr.eval(),

            Expr::Ternary(expr, if_, else_) => {
                let cond = expr.eval()?;

                if cond.is_truthy() {
                    if_.eval()
                } else {
                    else_.eval()
                }
            }
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Literal(literal) => literal.to_string(),
            Expr::Unary(op, r) => format!("({} {})", op.lexeme, r.to_string()),
            Expr::Binary(l, op, r) => {
                format!("({} {} {})", op.lexeme, l.to_string(), r.to_string())
            }
            Expr::Group(expr) => format!("(group {})", expr.to_string()),
            Expr::Ternary(expr, if_, else_) => format!(
                "({} ? {} : {})",
                expr.to_string(),
                if_.to_string(),
                else_.to_string()
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token_kind::TokenKind as TK;

    #[test]
    fn test_to_string() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::symbol(TK::Minus, "-".into(), 1),
                Box::new(Expr::Literal(Literal::Number(123.0))),
            )),
            Token::symbol(TK::Star, "*".into(), 1),
            Box::new(Expr::Group(Box::new(Expr::Literal(Literal::Number(45.67))))),
        );

        assert_eq!(expr.to_string(), "(* (- 123) (group 45.67))");
    }
}
