use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
	Expr(Expr),
	Print(Vec<Expr>),
	Var {
		name: Token,
		init: Expr,
	},
	Block {
		stmts: Vec<Stmt>,
	},
}
