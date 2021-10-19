use std::{
	env, fs,
	io::{self, Stdin, StdinLock, Stdout, StdoutLock, Write},
	sync::{Mutex, MutexGuard},
};

use bitflags::bitflags;
use itertools::Itertools;
use nu_ansi_term::{AnsiGenericString, Color};
use once_cell::sync::OnceCell;

use crate::debug::Repeat;

mod parser;

static STDIO: OnceCell<Stdio> = OnceCell::new();
lazy_static! {
	static ref DEBUG_FLAGS: Mutex<DebugFlags> = Mutex::new(DebugFlags::NONE);
}

#[derive(Debug)]
pub struct Args {
	pub example: Option<String>,
	pub debug: DebugFlags,
}

bitflags! {
	pub struct DebugFlags: u8 {
		const NONE    = 0b000;
		const PARSE   = 0b001;
		const CODEGEN = 0b010;
		const EXEC    = 0b100;

		const COMPILE = Self::PARSE.bits | Self::CODEGEN.bits;
	}
}

pub fn args() -> anyhow::Result<Args> {
	let raw = env::args().into_iter().skip(1).join("\n");
	let (_src, mut args) = parser::parse(raw)?;

	let mut flags = DEBUG_FLAGS.lock().unwrap();
	*flags = args.debug;
	drop(flags);

	if let Some(example) = args.example.as_mut() {
		let mut path = env::current_dir()?;
		path.extend(["packages", "spec", "src", "examples"]);
		path.push(format!("{}.lox", example));

		*example = fs::read_to_string(path)?;
	}

	Ok(args)
}

pub fn debug_flags<'a>() -> MutexGuard<'a, DebugFlags> {
	DEBUG_FLAGS.lock().unwrap()
}

pub fn stdout() -> StdoutLock<'static> {
	Stdio::get().stdout.lock()
}

pub fn stdin() -> StdinLock<'static> {
	Stdio::get().stdin.lock()
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

pub fn prompt_char() -> AnsiGenericString<'static, str> {
	Color::DarkGray.paint(format!("{}", '\u{276F}'))
}

pub fn cls() {
	print!("{esc}[2J{esc}[1;1H", esc = 0x1b as char);
}

pub fn print_header(title: &str, subhead: &str) {
	let divider = '='.repeat(60);
	let mut stdout = stdout();

	writeln!(stdout).unwrap();
	writeln!(
		stdout,
		"{} {} {}",
		Color::Yellow.bold().paint(title),
		prompt_char(),
		Color::LightBlue.paint(subhead),
	)
	.unwrap();
	writeln!(stdout, "{}", Color::DarkGray.paint(divider)).unwrap();

	stdout.flush().unwrap();
}
