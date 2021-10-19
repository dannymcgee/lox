use std::io::{self, prelude::*, StdoutLock};

use nu_ansi_term::Color;

use crate::{cli, vm};

pub fn start() -> anyhow::Result<()> {
	for line in Repl::start() {
		vm::get().interpret(line)?;
	}

	Ok(())
}

struct Repl;

impl Repl {
	fn start() -> Self {
		cli::cls();
		Self
	}
}

impl Iterator for Repl {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let mut stdout = cli::stdout();
		stdout.prompt().ok()?;
		stdout.flush().ok()?;

		drop(stdout);

		let mut buf = String::new();
		let mut stdin = cli::stdin();
		stdin.read_line(&mut buf).ok()?;

		Some(buf)
	}
}

trait ReplPrompt {
	fn prompt(&mut self) -> io::Result<()>;
}

impl ReplPrompt for StdoutLock<'static> {
	fn prompt(&mut self) -> io::Result<()> {
		write!(
			self,
			"{} {} ",
			Color::LightBlue.bold().paint("lox"),
			cli::prompt_char()
		)
	}
}
