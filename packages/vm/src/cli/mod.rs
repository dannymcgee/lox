use std::io::{self, Stdin, StdinLock, Stdout, StdoutLock};

use once_cell::sync::OnceCell;

pub use self::{
	args::{args, debug_flags, DebugFlags},
	fmt_colored::FmtColored,
	render::*,
};

mod args;
mod fmt_colored;
pub mod render;

static STDIO: OnceCell<Stdio> = OnceCell::new();

struct Stdio {
	stdin: Stdin,
	stdout: Stdout,
}

pub fn init() {
	let _ = Stdio::get();
}

pub fn stdout() -> StdoutLock<'static> {
	Stdio::get().stdout.lock()
}

pub fn stdin() -> StdinLock<'static> {
	Stdio::get().stdin.lock()
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
