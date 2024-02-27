#![allow(dead_code, unused_variables)]

mod token;
mod scanner;
mod literal;

use std::{cmp::Ordering, env, fs, io, path::Path};

fn read_input() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read user input");
	input.trim().to_owned()
}

fn report(line: usize, msg: &str, location: &str) {
	eprintln!("[line {line}] Error{location}: {msg}");
}

fn error(line: usize, msg: &str) {
	report(line, msg, "");
}

fn run(source: String) -> Result<(), String> {
	Err("Not yet implemented".to_owned())
}

fn run_file(path: &Path) -> Result<(), String> {
	let source = fs::read_to_string(path).expect(&format!("Could not open {}", path.display()));
	run(source)
}

fn run_prompt() {
	loop {
		print!("> ");

		let input = read_input();
		if input.is_empty() {
			break;
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
			// TODO: Replace 0 with the actual line number.
			Err(msg) => error(0, &msg),
		},
		Ordering::Less => run_prompt(),
	}
}
