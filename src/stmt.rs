use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
	Expr(Expr),
	Print(Expr),
	Var {
		name: Token,
		init: Expr,
	},
	Block(Vec<Stmt>),
}
