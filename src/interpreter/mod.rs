mod eval;
mod execute;

use self::execute::execute;
use crate::{error::runtime_error::RuntimeError, stmt::Stmt};

/// Executes multiple stataments, possibly causing side effects.
pub fn interpret(stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
    for stmt in stmts {
        execute(stmt)?;
    }

    Ok(())
}
