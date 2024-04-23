mod env;
mod eval;
mod execute;

use self::{env::Env, execute::execute};
use crate::{error::runtime_error::RuntimeError, stmt::Stmt};

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    /// Executes multiple stataments, possibly causing side effects.
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            execute(stmt, &mut self.env)?;
        }

        Ok(())
    }
}
