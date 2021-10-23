use std::{collections::VecDeque, fmt::Display, io::Write};

use crossterm::{cursor, queue, style, QueueableCommand};

use crate::debug::Repeat;

pub(super) struct View {
	state: VecDeque<String>,
	layout: Rect,
}

#[derive(Debug)]
pub(super) struct Rect {
	pub x: u16,
	pub y: u16,
	pub width: u16,
	pub height: u16,
}

impl View {
	pub(super) fn new(layout: Rect) -> Self {
		Self {
			state: VecDeque::new(),
			layout,
		}
	}

	pub(super) fn write_char<T>(
		&mut self,
		c: char,
		target: &mut T,
	) -> anyhow::Result<()>
	where
		T: QueueableCommand + Write,
	{
		let line_len = {
			let line = self.top_line();
			let len = line.len();
			line.push(c);
			len as u16
		};

		queue!(
			target,
			cursor::MoveTo(self.layout.x + line_len, self.layout.y),
			style::Print(c),
		)?;

		Ok(())
	}

	pub(super) fn write<S, T>(&mut self, word: S, target: &mut T) -> anyhow::Result<()>
	where
		S: Display,
		T: QueueableCommand + Write,
	{
		let line_len = {
			let line = self.top_line();
			let len = line.len();
			line.push_str(&word.to_string());
			len as u16
		};

		queue!(
			target,
			cursor::MoveTo(self.layout.x + line_len, self.layout.y),
			style::Print(word),
		)?;

		Ok(())
	}

	pub(super) fn writeln<S, T>(&mut self, data: S, target: &mut T) -> anyhow::Result<()>
	where
		S: Display,
		T: QueueableCommand + Write,
	{
		let line = data.to_string();

		self.shift(target)?;

		queue!(
			target,
			cursor::MoveTo(self.layout.x, self.layout.y),
			style::Print(&line),
		)?;

		self.clear_line_from(target, self.layout.x + line.len() as u16 + 1)?;
		*self.top_line() = line;

		Ok(())
	}

	fn top_line(&mut self) -> &mut String {
		if self.state.front_mut().is_none() {
			self.state.push_front(String::new());
		}

		self.state.front_mut().unwrap()
	}

	pub(super) fn shift<T>(&mut self, target: &mut T) -> anyhow::Result<()>
	where T: QueueableCommand {
		target.queue(cursor::MoveTo(self.layout.x, self.layout.y))?;
		self.clear_line(target)?;

		let mut r = self.layout.y + 1;
		let max = self.layout.y + self.layout.height;

		for line in self.state.iter() {
			let len = strip_ansi_escapes::strip(line)?.len();
			if r == max {
				break;
			}
			target.queue(cursor::MoveTo(self.layout.x, r))?;
			target.queue(style::Print(line))?;
			self.clear_line_from(target, self.layout.x + len as u16 + 1)?;

			r += 1;
		}

		if self.state.len() > self.layout.height as usize {
			let mut last = self.state.pop_back().unwrap();
			last.clear();
			self.state.push_front(last);
		} else {
			self.state.push_front(String::new());
		}

		Ok(())
	}

	fn clear_line<T>(&self, target: &mut T) -> anyhow::Result<()>
	where T: QueueableCommand {
		self.clear_line_from(target, self.layout.x)
	}

	fn clear_line_from<T>(&self, target: &mut T, col: u16) -> anyhow::Result<()>
	where T: QueueableCommand {
		let end = self.layout.x + self.layout.width - 2;
		let fill_len = (end as isize - col as isize).max(0) as u16;
		let fill = ' '.repeat(fill_len as usize);

		target.queue(cursor::MoveToColumn(col))?;
		target.queue(style::Print(fill))?;

		Ok(())
	}
}
