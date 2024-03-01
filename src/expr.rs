use crate::{literal::Literal, token::Token};

pub enum Expr {
	Literal(Literal),
	Unary(Token, Box<Expr>),
	Binary(Box<Expr>, Token, Box<Expr>),
	Group(Box<Expr>),
}

impl ToString for Expr {
	fn to_string(&self) -> String {
		match self {
			Expr::Literal(literal) => literal.to_string(),
			Expr::Unary(op, r) => format!("({} {})", op.lexeme, r.to_string()),
			Expr::Binary(l, op, r) => format!("({} {} {})", op.lexeme, l.to_string(), r.to_string()),
			Expr::Group(expr) => format!("(group {})", expr.to_string()),
		}

	}
}

#[cfg(test)]
mod test {
	use crate::token_kind::TokenKind;
	use super::*;

	#[test]
	fn test_to_string() {
		let expr = Expr::Binary(
			Box::new(
				Expr::Unary(
					Token::symbol(TokenKind::Minus, "-".into(), 1),
					Box::new(Expr::Literal(Literal::Number(123.0))),
				),
			),
			Token::symbol(TokenKind::Star, "*".into(), 1),
			Box::new(
				Expr::Group(
					Box::new(Expr::Literal(Literal::Number(45.67))),
				),
			),
		);

		assert_eq!(expr.to_string(), "(* (- 123) (group 45.67))");
	}
}
