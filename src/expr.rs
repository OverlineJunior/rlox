use crate::{literal::Literal, token::Token, token_kind::TokenKind as TK};

pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group(Box<Expr>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> Literal {
        match self {
            Self::Literal(lit) => lit.clone(),
            Self::Group(expr) => expr.eval(),

            Self::Unary(op, r) => match op.kind {
                TK::Minus => (-r.eval().expect_number()).into(),
                TK::Bang => (!r.eval().is_truthy()).into(),
                tk => panic!("`{:?}` is not an unary operator", tk),
            },

            Self::Binary(l, op, r) => match op.kind {
                TK::Plus => match (l.eval(), r.eval()) {
                    (Literal::Number(l), Literal::Number(r)) => (l + r).into(),
                    (Literal::String(l), Literal::String(r)) => (l + &r).into(),
                    (l, r) => panic!("Cannot add `{l}` with `{r}`"),
                },

                TK::Minus => l.eval() - r.eval(),
                TK::Star => l.eval() * r.eval(),
                TK::Slash => l.eval() / r.eval(),
                TK::Greater => (l.eval() > r.eval()).into(),
                TK::GreaterEqual => (l.eval() >= r.eval()).into(),
                TK::Less => (l.eval() < r.eval()).into(),
                TK::LessEqual => (l.eval() <= r.eval()).into(),
                TK::EqualEqual => (l.eval() == r.eval()).into(),
                TK::BangEqual => (l.eval() != r.eval()).into(),
                tk => panic!("`{:?}` is not a binary operator", tk),
            },

            Self::Ternary(expr, if_, else_) => {
                if expr.eval() == true.into() {
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
            },
            Expr::Group(expr) => format!("(group {})", expr.to_string()),
            Expr::Ternary(expr, if_, else_) => format!("({} ? {} : {})", expr.to_string(), if_.to_string(), else_.to_string()),
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
