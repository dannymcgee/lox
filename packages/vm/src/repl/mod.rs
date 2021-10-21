use std::io::{self, prelude::*, StdoutLock};

use nu_ansi_term::Color;

use crate::{
	cli::{self, FmtColored},
	vm,
};

pub fn start() -> anyhow::Result<()> {
	for line in Repl::start() {
		match vm::get().interpret(line)? {
			Some(value) => {
				let mut stdout = cli::stdout();
				writeln!(
					stdout,
					"{} {}",
					Color::DarkGray.paint("=>"),
					value.fmt_colored()
				)?;
			}
			None => {
				let mut stdout = cli::stdout();
				writeln!(stdout, "{}", Color::DarkGray.paint("=> void"),)?;
			}
		}
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

		writeln!(stdout).unwrap();
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
