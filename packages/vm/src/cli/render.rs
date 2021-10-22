use std::{
	collections::VecDeque,
	fmt::Write as _,
	io::{StdoutLock, Write as _},
	sync::{Mutex, TryLockError},
	thread::{self, JoinHandle},
	time::{Duration, Instant},
};

use itertools::Itertools;
use nu_ansi_term::{AnsiGenericString, Color};
use terminal_size::{Height, Width};

use crate::{debug::Repeat, repr::alloc::MemState};

lazy_static! {
	static ref STATE: Mutex<RenderState> = Mutex::new(RenderState::new());
	static ref PROMPT: String = prompt();
}

const VSYNC: Duration = Duration::from_nanos(8_333_333);

pub fn start() -> JoinHandle<()> {
	let size = terminal_size::terminal_size();

	thread::spawn(move || {
		let mut buf = String::new();

		loop {
			let start = Instant::now();

			match STATE.try_lock() {
				Ok(state) => {
					let mut stdout = super::stdout();
					let size = match size {
						Some((Width(w), Height(h))) => (w as usize, h as usize),
						None => panic!("Unable to get terminal dimensions"),
					};

					buf.clear();
					buf.print_prompt(&state.prompt);
					buf.print_main(state.body.iter(), state.debug.iter(), size);
					buf.print_footer(&state.footer, &start, size.0);

					clear(&mut stdout);
					write!(&mut stdout, "{}", buf).unwrap();
					stdout.flush().unwrap();
				}
				Err(TryLockError::Poisoned(_)) => {
					panic!("RenderState poisoned!");
				}
				_ => {}
			}

			thread::sleep(VSYNC - (Instant::now() - start));
		}
	})
}

struct RenderState {
	prompt: String,
	body: VecDeque<String>,
	debug: VecDeque<String>,
	footer: String,
}

impl RenderState {
	fn new() -> Self {
		Self {
			prompt: String::new(),
			body: VecDeque::new(),
			debug: VecDeque::new(),
			footer: String::new(),
		}
	}
}

trait Render<'a, 'b>
where 'a: 'b
{
	fn print_prompt(&mut self, prompt: &str);
	fn print_main<I>(&mut self, mut _body: I, mut _debug: I, _size: (usize, usize))
	where I: Iterator<Item = &'b String> {
	}
	fn print_footer(&mut self, footer: &str, frame_start: &Instant, width: usize);
}

impl<'a, 'b> Render<'a, 'b> for String
where 'a: 'b
{
	fn print_prompt(&mut self, prompt: &str) {
		write!(self, "{}", PROMPT.as_str()).unwrap();
		write!(self, "{}", prompt).unwrap();
		writeln!(self).unwrap();
	}

	fn print_main<I>(&mut self, mut body: I, mut debug: I, size: (usize, usize))
	where I: Iterator<Item = &'b String> {
		let (w, h) = size;
		let mid_col = ((w - 2) / 2) as isize;

		let body = body.join("\n");
		let debug = debug.join("\n");

		let line_count = usize::max(body.lines().count(), debug.lines().count());
		let fill_count = (h as isize - line_count as isize - 3).max(0) as usize;

		for fused in (body.lines()).zip_longest(debug.lines()) {
			use itertools::EitherOrBoth::*;

			let (body, debug) = match fused {
				Both(b, d) => (Some(b), Some(d)),
				Left(b) => (Some(b), None),
				Right(d) => (None, Some(d)),
			};

			if let Some(line) = body {
				let len = strip_ansi_escapes::strip(line).unwrap().len() as isize;
				let pad = ' '.repeat((mid_col - len).max(0) as usize);

				write!(self, "{}{}", line, pad).unwrap();
			} else {
				write!(self, "{}", ' '.repeat(mid_col as usize)).unwrap();
			}

			if let Some(line) = debug {
				writeln!(self, "{}", line).unwrap();
			} else {
				writeln!(self).unwrap();
			}
		}

		write!(self, "{}", '\n'.repeat(fill_count)).unwrap();
	}

	fn print_footer(&mut self, footer: &str, frame_start: &Instant, width: usize) {
		let footer_len = strip_ansi_escapes::strip(footer).unwrap().len();
		write!(self, "{}", footer).unwrap();

		let pad = ' '.repeat((width as isize - footer_len as isize - 10).max(0) as usize);
		let frame_time = (Instant::now() - *frame_start).as_micros() as f64 / 1000.;
		write!(self, "{}{:>10.3}", pad, frame_time).unwrap();
	}
}

pub fn prompt_char() -> AnsiGenericString<'static, str> {
	Color::DarkGray.paint(format!("{}", '\u{276F}'))
}

fn prompt() -> String {
	format!(
		"{} {} ",
		Color::LightBlue.bold().paint("lox"),
		prompt_char()
	)
}

fn clear(stdout: &mut StdoutLock) {
	write!(stdout, "{esc}[2J{esc}[1;1H", esc = 0x1b as char).unwrap();
}

pub fn cls() {
	print!("{esc}[2J{esc}[1;1H", esc = 0x1b as char);
}

pub fn print_header(title: &str, subhead: &str) {
	let divider = '='.repeat(60);
	let mut stdout = super::stdout();

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

pub fn update_mem_readout(state: MemState) {
	let footer = &mut STATE.lock().unwrap().footer;
	footer.clear();
	footer.push_str(&format!(
		"mem: {} | allocs: +{} / -{} ({})",
		fmt_bytes(state.bytes),
		state.allocs,
		state.deallocs,
		state.balance
	));
}

const KB: usize = 1024;
const MB: usize = 1024 * 1024;
const GB: usize = 1024 * 1024 * 1024;

fn fmt_bytes(size: usize) -> String {
	if size < KB {
		format!("{:.3} B", size)
	} else if size < MB {
		format!("{:.3} KB", size as f64 / KB as f64)
	} else if size < GB {
		format!("{:.3} MB", size as f64 / MB as f64)
	} else {
		format!("{:.3} GB", size as f64 / GB as f64)
	}
}
