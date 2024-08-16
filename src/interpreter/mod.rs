mod env;
mod eval;
mod execute;
pub mod runtime_error;

use self::{env::Env, execute::execute, runtime_error::RuntimeError};
use crate::parser::stmt::Stmt;

pub struct Interpreter {
    env: Env,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self { env: Env::new() }
    }
}

impl Interpreter {
    /// Executes multiple stataments, possibly causing side effects.
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            execute(stmt, &mut self.env)?;
        }

        Ok(())
    }
}
