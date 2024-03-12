use crate::{literal::Literal, token::Token};

pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Group(Token, Box<Expr>, Token),
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Literal(literal) => literal.to_string(),
            Expr::Unary(op, r) => format!("({} {})", op.lexeme, r.to_string()),
            Expr::Binary(l, op, r) => {
                format!("({} {} {})", op.lexeme, l.to_string(), r.to_string())
            }
            Expr::Group(l, expr, r) => format!("({}{}{})", l.lexeme, expr.to_string(), r.lexeme),
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
            Box::new(Expr::Group(
                Token::symbol(TK::LeftParenthesis, "(".into(), 1),
                Box::new(Expr::Literal(Literal::Number(45.67))),
                Token::symbol(TK::RightParenthesis, ")".into(), 1),
            )),
        );

        assert_eq!(expr.to_string(), "(* (- 123) (group 45.67))");
    }
}
