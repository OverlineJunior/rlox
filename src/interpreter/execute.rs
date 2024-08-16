use super::{env::Env, eval::eval, runtime_error::RuntimeError};
use crate::{parser::stmt::Stmt, scanner::literal::Literal};

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
            env.define(name, value);
        }

        Stmt::Block(stmts) => {
            let mut new_env = Env::new_enclosed(env.clone());

            for stmt in stmts {
                execute(stmt, &mut new_env)?;
            }
        }
    };

    Ok(())
}
