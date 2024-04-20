use super::eval::eval;
use crate::{error::runtime_error::RuntimeError, stmt::Stmt};

/// Executes a single statament tree, possibly causing side effects.
/// This is the statement analogue of `eval`.
pub fn execute(stmt: Stmt) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Expr(expr) => {
            eval(expr)?;
        }

        Stmt::Print(expr) => println!("{}", eval(expr)?),

        Stmt::Var { name, init } => todo!("execute"),
    };

    Ok(())
}
