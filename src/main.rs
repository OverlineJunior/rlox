#![allow(dead_code, unused_variables, unused_imports)]

mod expr;
mod literal;
mod scanner;
mod token;
mod token_kind;
mod cursor;
mod string_cursor;
mod parser;
mod error;

use std::{cmp::Ordering, env, fs, io, path::Path};

use error::Error;
use parser::parse;
use scanner::tokenize;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    input.trim().to_owned()
}

fn run(source: String) -> Result<(), Error> {
    let tokens = tokenize(source)?;
    let expr = parse(tokens)?;

    println!("> {:?}", expr.to_string());

    Ok(())
}

fn run_file(path: &Path) -> Result<(), Error> {
    let source =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open {}", path.display()));
    run(source)
}

fn run_prompt() {
    println!("rlox (Ctrl+C to exit)");

    loop {
        let input = read_input();
        if input.is_empty() {
            continue;
        }

        if let Err(msg) = run(input) {
            println!("Error: {msg}");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 2 instead of 1 because the first value of args is not an user argument.
    match args.len().cmp(&2) {
        Ordering::Greater => panic!("Usage: rlox [script]"),
        Ordering::Equal => match run_file(Path::new(&args[1])) {
            Ok(_) => (),
            Err(err) => eprintln!("Error: {err}"),
        },
        Ordering::Less => run_prompt(),
    }
}
