use std::fmt::Display;

use crate::{literal::Literal, stmt::Stmt, token::Token};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group(Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Variable { name: Token },
    Assign { name: Token, value: Box<Expr> },
    Block { stmts: Vec<Stmt> },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(literal) => write!(f, "{}", literal),

            Expr::Unary(op, r) => write!(f, "({} {})", op.lexeme, r),

            Expr::Binary(l, op, r) => {
                write!(f, "({} {} {})", op.lexeme, l, r)
            }

            Expr::Group(expr) => write!(f, "(group {})", expr),

            Expr::Ternary(expr, if_, else_) => write!(f, "({} ? {} : {})", expr, if_, else_),

            Expr::Variable { name } => write!(f, "(var {})", name.lexeme),

            Expr::Assign { name, value } => write!(f, "(assign {} = {})", name.lexeme, value,),

            Expr::Block { stmts } => write!(f, "(block ...)"),
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
