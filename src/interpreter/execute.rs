use super::{env::Env, eval::eval};
use crate::{error::runtime_error::RuntimeError, literal::Literal, stmt::Stmt};

/// Executes a single statament tree, possibly causing side effects.
/// This is the statement analogue of `eval`.
pub fn execute(stmt: Stmt, env: &mut Env) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Expr(expr) => {
            eval(expr, env)?;
        }

        Stmt::Print(expr) => println!("{}", eval(expr, env)?),

        Stmt::Var { name, init } => {
            let value = eval(init, env)?;
            env.define(&name.lexeme, value);
        }
    };

    Ok(())
}
