use std::{
	fmt::Display,
	io::{self, BufWriter, Write},
	process,
	sync::mpsc::{self, Receiver, Sender},
	time::Duration,
};

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
	execute, queue,
	style::{self, Attribute, Color},
	terminal::{self, ClearType},
	QueueableCommand,
};

use crate::{debug::Repeat, repr::alloc::MemState};

use super::view::{Rect, View};

pub enum Area {
	Output,
	Debug,
}

pub struct Stdio {
	target: BufWriter<io::Stdout>,
	input: String,
	cursor: u16,
	stdin_rx: Option<Receiver<String>>,
	stdin_tx: Sender<String>,
	output: View,
	debug: View,
	dirty: bool,
	size: (u16, u16),
}

impl Stdio {
	pub(super) fn new() -> Self {
		let (w, h) = terminal::size().unwrap();

		let output_width = w / 2;
		let debug_width = w - output_width;
		let view_height = h - 5;

		let output = View::new(Rect {
			x: 0,
			y: 2,
			width: output_width,
			height: view_height,
		});

		let debug = View::new(Rect {
			x: output_width,
			y: 2,
			width: debug_width,
			height: view_height,
		});

		let (stdin_tx, stdin_rx) = mpsc::channel();

		Self {
			target: BufWriter::with_capacity(w as usize * h as usize, io::stdout()),
			input: String::with_capacity(w as usize - 6),
			cursor: 0,
			stdin_rx: Some(stdin_rx),
			stdin_tx,
			output,
			debug,
			dirty: true,
			size: (w, h),
		}
	}

	pub fn stdin(&mut self) -> Option<Receiver<String>> {
		self.stdin_rx.take()
	}

	pub fn writeln<S>(&mut self, data: S, target: Area) -> anyhow::Result<()>
	where S: Display {
		self.mark_dirty()?;
		match target {
			Area::Output => self.output.writeln(data, &mut self.target),
			Area::Debug => self.debug.writeln(data, &mut self.target),
		}
	}

	#[allow(dead_code)]
	pub fn write<S>(&mut self, data: S, target: Area) -> anyhow::Result<()>
	where S: Display {
		self.mark_dirty()?;
		match target {
			Area::Output => self.output.write(data, &mut self.target),
			Area::Debug => self.debug.write(data, &mut self.target),
		}
	}

	#[allow(dead_code)]
	pub fn write_char(&mut self, c: char, target: Area) -> anyhow::Result<()> {
		self.mark_dirty()?;
		match target {
			Area::Output => self.output.write_char(c, &mut self.target),
			Area::Debug => self.debug.write_char(c, &mut self.target),
		}
	}

	pub fn flush(&mut self) -> anyhow::Result<()> {
		let target = &mut self.target;
		queue!(target, cursor::RestorePosition, style::ResetColor)?;
		target.flush()?;

		self.dirty = false;

		Ok(())
	}

	#[allow(dead_code)]
	pub fn endl(&mut self, target: Area) -> anyhow::Result<()> {
		self.mark_dirty()?;
		match target {
			Area::Output => self.output.shift(&mut self.target),
			Area::Debug => self.output.shift(&mut self.target),
		}
	}

	pub fn update_mem_readout(&mut self, state: MemState) -> anyhow::Result<()> {
		self.mark_dirty()?;

		let (w, h) = self.size;
		let target = &mut self.target;
		let content = format!(
			"mem: {} | allocations: {}",
			fmt_bytes(state.bytes),
			state.allocs
		);
		let clear_fill = ' '.repeat(content.len() + 10);
		let col = (w as isize - content.len() as isize).max(0) as u16;

		queue!(
			target,
			cursor::MoveTo(col, h - 2),
			style::Print(clear_fill),
			cursor::MoveTo(col, h - 2),
			style::SetForegroundColor(Color::DarkGrey),
			style::Print(content),
		)?;

		self.flush()?;

		Ok(())
	}

	pub(super) fn init_prompt(&mut self) -> anyhow::Result<()> {
		self.clear()?;

		let (w, h) = self.size;
		let divider = '\u{2014}'.repeat(w as usize);
		let target = &mut self.target;

		queue!(
			target,
			style::SetForegroundColor(Color::DarkGrey),
			cursor::MoveTo(0, 1),
			style::Print(&divider),
			cursor::MoveTo(0, h - 3),
			style::Print(divider),
			cursor::MoveTo(0, h - 2),
			style::Print(callout("Scroll Output", "Ctrl+Up/Down")),
			style::ResetColor,
			style::Print("    "),
			style::Print(callout("Scroll Debug", "Ctrl+Shift+Up/Down")),
			style::ResetColor,
			cursor::MoveTo(0, 0),
			style::SetForegroundColor(Color::Blue),
			style::SetAttribute(Attribute::Bold),
			style::Print("lox"),
			style::SetAttribute(Attribute::Reset),
			style::SetForegroundColor(Color::DarkGrey),
			style::Print(' '),
			style::Print('\u{276F}'),
			style::Print(' '),
			cursor::SavePosition,
		)?;

		self.flush()
	}

	pub(super) fn poll_events(&mut self) -> anyhow::Result<()> {
		if event::poll(Duration::from_millis(5))? {
			match event::read()? {
				Event::Key(KeyEvent { code, modifiers }) => {
					use KeyCode::*;

					if matches!(code, Char('c'))
						&& modifiers.contains(KeyModifiers::CONTROL)
					{
						self.exit();
					}

					queue!(&mut self.target, style::ResetColor)?;

					match code {
						Backspace => self.backspace()?,
						Enter => self.submit_stdin()?,
						Left => self.cursor_left()?,
						Right => self.cursor_right()?,
						Up => self.scroll_kb(1, modifiers)?,
						Down => self.scroll_kb(-1, modifiers)?,
						Home => self.home()?,
						End => self.end()?,
						PageUp => {}   // TODO
						PageDown => {} // TODO
						Tab => {}      // TODO
						BackTab => {}  // TODO
						Delete => self.delete()?,
						Insert => {} // TODO
						F(_) => {}   // TODO
						Char(c) => self.key(c)?,
						Null => {} // TODO
						Esc => self.exit(),
					}
				}
				Event::Mouse(evt @ MouseEvent { kind, .. }) => match kind {
					MouseEventKind::ScrollDown => self.scroll_mouse(-1, evt)?,
					MouseEventKind::ScrollUp => self.scroll_mouse(1, evt)?,
					_ => {}
				},
				Event::Resize(_, _) => {} // TODO
			}
			queue!(&mut self.target, cursor::SavePosition)?;
			self.flush()?;
		}

		Ok(())
	}

	fn backspace(&mut self) -> anyhow::Result<()> {
		if self.cursor == self.input.len() as u16 && self.input.pop().is_some() {
			self.cursor -= 1;

			queue!(
				&mut self.target,
				cursor::MoveLeft(1),
				style::Print(' '),
				cursor::MoveLeft(1),
			)?;
		} else if !self.input.is_empty() {
			self.cursor -= 1;
			let cursor = self.cursor;

			self.input.remove(cursor as usize);
			let mut rem = (&self.input[cursor as usize..]).to_owned();
			rem.push(' ');

			queue!(
				&mut self.target,
				cursor::MoveLeft(1),
				style::Print(rem),
				cursor::MoveTo(cursor + 6, 0),
			)?;
		}

		Ok(())
	}

	fn submit_stdin(&mut self) -> anyhow::Result<()> {
		self.stdin_tx.send(self.input.clone()).unwrap();
		self.cursor = 0;
		self.input.clear();

		let (width, _) = self.size;

		queue!(
			&mut self.target,
			cursor::MoveTo(6, 0),
			style::Print(' '.repeat(width as usize - 6)),
			cursor::MoveTo(6, 0),
		)?;

		Ok(())
	}

	fn cursor_left(&mut self) -> anyhow::Result<()> {
		// TODO: jump between words when Ctrl is held
		if self.cursor > 0 {
			self.cursor -= 1;
			queue!(&mut self.target, cursor::MoveLeft(1))?;
		}

		Ok(())
	}

	fn cursor_right(&mut self) -> anyhow::Result<()> {
		// TODO: jump between words when Ctrl is held
		if self.cursor < self.input.len() as u16 {
			self.cursor += 1;
			queue!(&mut self.target, cursor::MoveRight(1))?;
		}

		Ok(())
	}

	fn home(&mut self) -> anyhow::Result<()> {
		self.cursor = 0;
		queue!(&mut self.target, cursor::MoveTo(6, 0))?;
		Ok(())
	}

	fn end(&mut self) -> anyhow::Result<()> {
		self.cursor = self.input.len() as u16;
		let input_len = self.input.len();

		queue!(&mut self.target, cursor::MoveTo(input_len as u16 + 6, 0))?;
		Ok(())
	}

	fn delete(&mut self) -> anyhow::Result<()> {
		if !self.input.is_empty() {
			let cursor = self.cursor;
			let (width, _) = self.size;

			if cursor == self.input.len() as u16 - 1 {
				let _ = self.input.pop();
				queue!(&mut self.target, style::Print(' '), cursor::MoveLeft(1))?;
			} else {
				self.input.remove(cursor as usize);

				let del_len = width - cursor - 6;
				let fill = ' '.repeat(del_len as usize);
				let rem = (&self.input[cursor as usize..]).to_owned();

				queue!(
					&mut self.target,
					cursor::MoveTo(cursor + 6, 0),
					style::Print(fill),
					cursor::MoveTo(cursor + 6, 0),
					style::Print(rem),
					cursor::MoveTo(cursor + 6, 0),
				)?;
			}
		}

		Ok(())
	}

	fn key(&mut self, key: char) -> anyhow::Result<()> {
		let cursor = self.cursor;
		self.cursor += 1;

		if cursor == self.input.len() as u16 {
			self.input.push(key);
			queue!(&mut self.target, style::Print(key))?;
		} else {
			self.input.insert(cursor as usize, key);
			let rem = (&self.input[cursor as usize..]).to_owned();
			queue!(
				&mut self.target,
				style::Print(rem),
				cursor::MoveTo(cursor + 7, 0)
			)?;
		}

		Ok(())
	}

	/// `delta` is -1 to reveal past messages, or +1 to reveal newer.
	/// Since messages are displayed in reverse chrono order, ScrollDown == -1.
	fn scroll_mouse(&mut self, delta: i8, event: MouseEvent) -> anyhow::Result<()> {
		let view = if self.output.contains(event.column, event.row) {
			self.mark_dirty()?;
			Some(&mut self.output)
		} else if self.debug.contains(event.column, event.row) {
			self.mark_dirty()?;
			Some(&mut self.debug)
		} else {
			None
		};

		if let Some(view) = view {
			view.scroll(delta, &mut self.target)?;
			self.flush()?;
		}

		Ok(())
	}

	fn scroll_kb(&mut self, delta: i8, mods: KeyModifiers) -> anyhow::Result<()> {
		let view = if mods.contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT) {
			self.mark_dirty()?;
			Some(&mut self.debug)
		} else if mods.intersects(KeyModifiers::CONTROL) {
			self.mark_dirty()?;
			Some(&mut self.output)
		} else {
			None
		};

		if let Some(view) = view {
			view.scroll(delta, &mut self.target)?;
			self.flush()?;
		}

		Ok(())
	}

	fn clear(&mut self) -> anyhow::Result<()> {
		write!(self.target, "{esc}[2J{esc}[1;1H", esc = 0x1b as char)?;
		Ok(())
	}

	fn mark_dirty(&mut self) -> anyhow::Result<()> {
		if !self.dirty {
			self.target.queue(cursor::SavePosition)?;
			self.dirty = true;
		}

		Ok(())
	}

	fn exit(&mut self) {
		execute!(&mut self.target, terminal::Clear(ClearType::All)).unwrap();
		process::exit(0);
	}
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

fn callout(name: &str, binding: &str) -> String {
	use nu_ansi_term::Color;

	let name = Color::DarkGray
		.reverse()
		.paint(format!(" {} ", name));
	let binding = Color::DarkGray.paint(binding);

	format!("{} {}", name, binding)
}
