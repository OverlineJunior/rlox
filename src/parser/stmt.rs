use super::expr::Expr;
use crate::scanner::token::Token;

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
