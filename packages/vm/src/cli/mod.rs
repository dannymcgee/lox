use std::{
	env, fmt, fs,
	io::{self, Stdin, StdinLock, Stdout, StdoutLock},
};

use itertools::Itertools;
use nu_ansi_term::{AnsiGenericString, Color};
use once_cell::sync::OnceCell;

mod parser;

static STDIO: OnceCell<Stdio> = OnceCell::new();

#[derive(Debug)]
pub struct Args {
	pub example: Option<String>,
}

pub fn args() -> anyhow::Result<Args> {
	let raw = env::args().into_iter().skip(1).join("\n");
	let mut args = parser::parse(raw)?;

	if let Some(example) = args.example.as_mut() {
		let mut path = env::current_dir()?;
		path.extend(["packages", "spec", "src", "examples"]);
		path.push(format!("{}.lox", example));

		*example = fs::read_to_string(path)?;
	}

	Ok(args)
}

struct Stdio {
	stdin: Stdin,
	stdout: Stdout,
}

impl Stdio {
	fn new() -> Self {
		Stdio {
			stdin: io::stdin(),
			stdout: io::stdout(),
		}
	}

	fn get() -> &'static Self {
		STDIO.get_or_init(Stdio::new)
	}
}

pub fn stdout() -> StdoutLock<'static> {
	Stdio::get().stdout.lock()
}

pub fn stdin() -> StdinLock<'static> {
	Stdio::get().stdin.lock()
}

pub fn prompt_char() -> AnsiGenericString<'static, str> {
	dimmed('\u{276F}')
}

pub fn dimmed<S>(content: S) -> AnsiGenericString<'static, str>
where S: fmt::Display {
	Color::DarkGray.paint(format!("{}", content))
}

pub fn debug_dimmed<S>(content: S) -> AnsiGenericString<'static, str>
where S: fmt::Debug {
	Color::DarkGray.paint(format!("{:?}", content))
}

pub fn cls() {
	print!("{esc}[2J{esc}[1;1H", esc = 0x1b as char);
}
