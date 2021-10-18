use std::io::{self, prelude::*, Stdin, Stdout};

use crate::vm;

pub fn start() -> anyhow::Result<()> {
	for line in Repl::start() {
		vm::get().interpret(line)?;
	}

	Ok(())
}

struct Repl {
	stdin: Stdin,
	stdout: Stdout,
}

impl Repl {
	fn start() -> Self {
		// Clear the terminal
		print!("{esc}[2J{esc}[1;1H", esc = 0x1b as char);

		Self {
			stdin: io::stdin(),
			stdout: io::stdout(),
		}
	}
}

impl Iterator for Repl {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let mut stdout = self.stdout.lock();
		write!(stdout, "lox > ").ok()?;
		stdout.flush().ok()?;

		drop(stdout);

		let mut buf = String::new();
		let mut stdin = self.stdin.lock();
		stdin.read_line(&mut buf).ok()?;

		Some(buf)
	}
}
