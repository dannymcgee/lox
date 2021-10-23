use std::{collections::VecDeque, fmt::Display, io::Write};

use crossterm::{cursor, queue, style, QueueableCommand};

use crate::debug::Repeat;

pub(super) struct View {
	messages: VecDeque<String>,
	top: usize,
	max: usize,
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
		let max = layout.height as usize * 10;

		Self {
			messages: VecDeque::with_capacity(max),
			top: 0,
			max,
			layout,
		}
	}

	pub(super) fn contains(&self, col: u16, row: u16) -> bool {
		col >= self.layout.x
			&& col < self.layout.x + self.layout.width
			&& row >= self.layout.y
			&& row < self.layout.y + self.layout.height
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
			let len = strip_ansi_escapes::strip(&line)?.len();
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
		let line_len = strip_ansi_escapes::strip(&line)?.len();

		self.shift(target)?;

		queue!(
			target,
			cursor::MoveTo(self.layout.x, self.layout.y),
			style::Print(&line),
		)?;

		self.clear_line_from(target, self.layout.x + line_len as u16 + 1)?;
		*self.top_line() = line;

		Ok(())
	}

	pub(super) fn shift<T>(&mut self, target: &mut T) -> anyhow::Result<()>
	where T: QueueableCommand {
		target.queue(cursor::MoveTo(self.layout.x, self.layout.y))?;
		self.clear_line(target)?;

		let mut r = self.layout.y + 1;
		let max = self.layout.y + self.layout.height;

		for line in self.messages.iter() {
			let len = strip_ansi_escapes::strip(line)?.len();
			if r == max {
				break;
			}
			target.queue(cursor::MoveTo(self.layout.x, r))?;
			target.queue(style::Print(line))?;
			self.clear_line_from(target, self.layout.x + len as u16 + 1)?;

			r += 1;
		}

		if self.messages.len() == self.max {
			let mut last = self.messages.pop_back().unwrap();
			last.clear();
			self.messages.push_front(last);
		} else {
			self.messages.push_front(String::new());
		}

		Ok(())
	}

	/// `delta` is -1 to reveal older messages, +1 to reveal newer
	pub(super) fn scroll<T>(&mut self, delta: i8, target: &mut T) -> anyhow::Result<()>
	where T: QueueableCommand {
		let height = self.layout.height as usize;
		if self.messages.len() <= height {
			return Ok(());
		}

		let max = self.messages.len();
		let top_max = (max as isize - height as isize).max(0);
		let new_top = (self.top as isize - delta as isize).clamp(0, top_max) as usize;

		if new_top == self.top {
			return Ok(());
		} else {
			self.top = new_top;
		}

		self.messages.make_contiguous();
		let (messages, _) = self.messages.as_slices();

		let mut r = self.layout.y;
		for line in &messages[self.top..self.top + height] {
			let len = strip_ansi_escapes::strip(line)?.len();

			target.queue(cursor::MoveTo(self.layout.x, r))?;
			target.queue(style::Print(line))?;
			self.clear_line_from(target, self.layout.x + len as u16 + 1)?;

			r += 1;
		}

		Ok(())
	}

	fn top_line(&mut self) -> &mut String {
		if self.messages.is_empty() {
			self.messages.push_front(String::new());
		}
		self.messages.front_mut().unwrap()
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
