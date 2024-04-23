mod eval;
mod execute;
mod env;

use self::{env::Env, execute::execute};
use crate::{error::runtime_error::RuntimeError, stmt::Stmt};

/// Executes multiple stataments, possibly causing side effects.
pub fn interpret(stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
    let mut env = Env::new();

    for stmt in stmts {
        execute(stmt, &mut env)?;
    }

    Ok(())
}
