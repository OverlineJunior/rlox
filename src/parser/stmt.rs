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
	If {
		condition: Expr,
		then_branch: Box<Stmt>,
		else_branch: Option<Box<Stmt>>,
	},
}
