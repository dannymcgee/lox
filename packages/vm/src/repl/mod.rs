use std::sync::mpsc::Receiver;

use nu_ansi_term::Color;

use crate::{
	cli::{self, Area, FmtColored},
	vm,
};

pub fn start() -> anyhow::Result<()> {
	for line in Repl::start() {
		match vm::get().interpret(line.clone())? {
			Some(value) => {
				let mut stdio = cli::stdio();
				stdio.writeln(
					format!(
						"{} {} {}",
						Color::DarkGray.paint(line),
						Color::DarkGray.paint("=>"),
						value.fmt_colored(),
					),
					Area::Output,
				)?;
				stdio.flush()?;
			}
			None => {
				let mut stdio = cli::stdio();
				stdio.writeln(Color::DarkGray.paint("=> void"), Area::Output)?;
				stdio.flush()?;
			}
		}
	}

	Ok(())
}

struct Repl {
	stdin: Receiver<String>,
}

impl Repl {
	fn start() -> Self {
		Self {
			stdin: cli::stdio().stdin().unwrap(),
		}
	}
}

impl Iterator for Repl {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.stdin.recv().ok()
	}
}
