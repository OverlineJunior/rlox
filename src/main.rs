#![allow(dead_code, unused_variables, unused_imports)]

pub mod cursor;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;

use std::{cmp::Ordering, env, fs, io, path::Path};

use error::Error;
use interpreter::Interpreter;
use parser::parse;
use scanner::tokenize;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_owned()
}

fn run(source: String, interpreter: &mut Interpreter) -> Result<(), Error> {
    let tokens = tokenize(source)?;
    let stmts = parse(tokens)?;

    interpreter.interpret(stmts)?;
    Ok(())
}

fn run_file(path: &Path, interpreter: &mut Interpreter) -> Result<(), Error> {
    let source =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open {}", path.display()));
    run(source, interpreter)
}

fn run_prompt(interpreter: &mut Interpreter) {
    println!("rlox (Ctrl+C to exit)");

    loop {
        let input = read_input();
        if input.is_empty() {
            continue;
        }

        if let Err(msg) = run(input, interpreter) {
            println!("Error: {msg}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::default();

    // 2 instead of 1 because the first value of args is not an user argument.
    match args.len().cmp(&2) {
        Ordering::Greater => panic!("Usage: rlox [script]"),
        Ordering::Equal => match run_file(Path::new(&args[1]), &mut interpreter) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {err}"),
        },
        Ordering::Less => run_prompt(&mut interpreter),
    }
}
