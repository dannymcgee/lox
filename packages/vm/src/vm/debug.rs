#[cfg(debug_assertions)]
use std::{
	cell::UnsafeCell,
	fmt::{Alignment, Write},
};

use nu_ansi_term::Color;
use strip_ansi_escapes as ansi;

#[cfg(debug_assertions)]
use Alignment::*;

use crate::{
	chunk::{Lines, OpCode},
	cli::{self, Area, DebugFlags, FmtColored},
	repr::Value,
	stack::Stack,
};

#[cfg(debug_assertions)]
use crate::debug::Repeat;

#[cfg(debug_assertions)]
pub(super) struct Disassembler {
	buf: UnsafeCell<String>,
	col: UnsafeCell<isize>,
}

// TODO - Replace messy crate::debug module with this

#[cfg(debug_assertions)]
impl Disassembler {
	// Column offsets for the output
	const ADDR: isize = 0;
	const LINE: isize = 12;
	const INSTR: isize = 14;
	const STACK: isize = 45;

	pub fn new() -> Self {
		Self {
			buf: UnsafeCell::new(String::new()),
			col: UnsafeCell::new(Self::ADDR),
		}
	}

	pub fn write_header(&self, _: &str) {
		if cli::debug_flags().contains(DebugFlags::EXEC) {
			let mut stdio = cli::stdio();
			stdio.writeln("", Area::Debug).unwrap();
			stdio.flush().unwrap();
		}
	}

	pub fn write_preamble(&self, offset: usize, lines: &Lines) {
		self.write_offset(offset);
		self.write_line(offset, lines);
	}

	fn write_offset(&self, offset: usize) {
		self.set_col(Self::ADDR);

		let data = Color::DarkGray
			.paint(format!("{:#06x}", offset))
			.to_string();

		self.write(data, Left);
	}

	fn write_line(&self, offset: usize, lines: &Lines) {
		self.set_col(Self::LINE);

		let line = lines.find_line(offset);
		let prev_line = if offset > 0 {
			Some(lines.find_line(offset - 1))
		} else {
			None
		};

		let data = match prev_line {
			Some(prev) if prev == line => Color::DarkGray.paint("|").to_string(),
			_ => line.to_string(),
		};

		self.write(data, Right);
	}

	pub fn write_opcode(&self, op: OpCode) {
		use OpCode::*;

		self.set_col(Self::INSTR);

		let byte = Color::DarkGray.paint(format!("{:#04x}", op as u8));

		let name = format!("{:?}", op);
		let name = match op {
			Constant | Constant16 | Constant24 => Color::Green.paint(name),
			True | False | Nil => Color::Cyan.paint(name),
			_ => Color::Fixed(5).bold().paint(name),
		};

		self.write(format!("{} {}", byte, name), Left);
	}

	pub fn write_value(&self, value: &Value) {
		let data = format!(" {}", Color::Cyan.paint(value.fmt_colored()));
		self.write(data, Left);
	}

	pub fn write_stack(&self, stack: &Stack<Value>) {
		self.set_col(Self::STACK);

		let data = format!("{:?}", stack);
		self.write(data, Left);
	}

	pub fn flush(&self) {
		let buf = self.buf();
		if cli::debug_flags().contains(DebugFlags::EXEC) {
			let mut stdio = cli::stdio();
			stdio.writeln(buf.clone(), Area::Debug).unwrap();
			stdio.flush().unwrap();
		}
		buf.clear();
	}

	fn write(&self, content: String, align: Alignment) {
		let content_len = ansi::strip(&content).unwrap().len() as isize;
		let buf_len = self.buf_len();
		let buf = self.buf();

		match align {
			Left => {
				let pad = (self.col() - buf_len).max(0) as usize;
				write!(buf, "{}{}", ' '.repeat(pad), content).unwrap();
			}
			Right => {
				let pad = (self.col() - buf_len - content_len).max(0) as usize;
				write!(buf, "{}{}", ' '.repeat(pad), content).unwrap();
			}
			Center => unimplemented!("Center alignment not supported"),
		}
	}

	#[inline]
	#[allow(clippy::mut_from_ref)]
	fn buf(&self) -> &mut String {
		unsafe { &mut *self.buf.get() }
	}

	#[inline]
	fn buf_len(&self) -> isize {
		let buf = unsafe { &*self.buf.get() };
		ansi::strip(buf).unwrap().len() as _
	}

	#[inline]
	fn col(&self) -> isize {
		unsafe { *self.col.get() }
	}

	#[inline]
	fn set_col(&self, col: isize) {
		let c = unsafe { &mut *self.col.get() };
		*c = col;
	}
}

#[cfg(not(debug_assertions))]
pub(super) struct Disassembler;

#[cfg(not(debug_assertions))]
#[rustfmt::skip]
impl Disassembler {
	#[inline(always)] pub fn new() -> Self { Self }
	#[inline(always)] pub fn write_header(&self, _: &str) {}
	#[inline(always)] pub fn write_preamble(&self, _: usize, _: &Lines) {}
	#[inline(always)] pub fn write_opcode(&self, _: OpCode) {}
	#[inline(always)] pub fn write_value(&self, _: Value) {}
	#[inline(always)] pub fn write_stack(&self, _: &Stack<Value>) {}
	#[inline(always)] pub fn flush(&self) {}
}
