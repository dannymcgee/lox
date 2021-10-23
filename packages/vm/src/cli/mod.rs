use std::{
	io,
	sync::{Mutex, MutexGuard},
	thread::{self, JoinHandle},
	time::Duration,
};

use crossterm::{
	cursor, execute,
	terminal::{self, ClearType},
};

pub use self::{
	args::{args, debug_flags, DebugFlags},
	fmt_colored::FmtColored,
	stdio::*,
};

mod args;
mod fmt_colored;
mod stdio;
mod view;

lazy_static! {
	static ref STDIO: Mutex<Stdio> = Mutex::new(Stdio::new());
}

pub fn init() -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
	terminal::enable_raw_mode()?;

	let mut stderr = io::stderr_locked();
	execute!(
		&mut stderr,
		terminal::Clear(ClearType::All),
		cursor::MoveTo(0, 0),
	)?;
	drop(stderr);

	stdio().init_prompt()?;

	let handle = thread::spawn(|| loop {
		stdio().poll_events()?;
		thread::sleep(Duration::from_millis(5));
	});

	Ok(handle)
}

pub fn stdio<'a>() -> MutexGuard<'a, Stdio> {
	STDIO.lock().unwrap()
}
