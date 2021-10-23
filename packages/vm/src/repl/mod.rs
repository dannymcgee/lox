use std::sync::mpsc::Receiver;

use nu_ansi_term::Color;

use crate::{
	cli::{self, Area, FmtColored},
	vm,
};

pub fn start() -> anyhow::Result<()> {
	for line in Repl::start() {
		let result = vm::get().interpret(line.clone());
		let mut stdio = cli::stdio();

		stdio.endl(Area::Output)?;
		stdio.write(
			format!(
				"{} {} ",
				Color::DarkGray.paint(line),
				Color::DarkGray.paint("=>")
			),
			Area::Output,
		)?;

		match result {
			Ok(Some(value)) => {
				stdio.write(value.fmt_colored(), Area::Output)?;
			}
			Ok(None) => {
				stdio.write(Color::DarkGray.italic().paint("void"), Area::Output)?;
			}
			Err(err) => {
				stdio.write(Color::Red.bold().paint("ERROR"), Area::Output)?;
				stdio.writeln("", Area::Debug)?;

				for line in err
					.to_string()
					.lines()
					.rev()
					.filter(|l| !l.is_empty())
				{
					stdio.writeln(Color::Red.paint(line), Area::Debug)?;
				}
			}
		}

		stdio.flush()?;
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
