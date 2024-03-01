use crate::{literal::Literal, token::Token};

pub enum Expr {
	Literal(Literal),
	Unary(Token, Box<Expr>),
	Binary(Box<Expr>, Token, Box<Expr>),
	Group(Box<Expr>),
}
