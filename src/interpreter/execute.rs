use std::{cell::RefCell, rc::Rc};

use super::{env::Env, eval::eval, runtime_error::RuntimeError};
use crate::{parser::stmt::Stmt, scanner::{literal::Literal, token::Token, token_kind::TokenKind}};

/// Executes a single statament tree, possibly causing side effects.
/// This is the statement analogue of `eval`.
pub fn execute(stmt: Stmt, env: Rc<RefCell<Env>>) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Expr(expr) => {
            eval(expr, env)?;
        }

        Stmt::Print(expr) => println!("{}", eval(expr, env)?),

        Stmt::Var { name, init } => {
            let value = eval(init, env.clone())?;
            env.borrow_mut().define(name, value);
        }

        Stmt::Block(stmts) => {
            let new_env = Env::new_enclosed(&env);

            for stmt in stmts {
                execute(stmt, new_env.clone())?;
            }
        },

        Stmt::If { condition, then_branch, else_branch } => {
            if eval(condition, env.clone())?.is_truthy() {
                execute(*then_branch, env)?;
            } else if let Some(else_branch) = else_branch {
                execute(*else_branch, env)?;
            }
        }

        Stmt::While { condition, body } => {
            while eval(condition.clone(), env.clone())?.is_truthy() {
                execute(*body.clone(), env.clone())?;
            }
        }
    };

    Ok(())
}
