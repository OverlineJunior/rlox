use crate::expr::Expr;

#[derive(Debug)]
pub enum Stmt {
	Expr(Expr),
	Print(Expr),
}
