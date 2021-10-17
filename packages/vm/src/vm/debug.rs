#[cfg(debug_assertions)]
use std::{
	cell::UnsafeCell,
	fmt::{Alignment, Write},
};

#[cfg(debug_assertions)]
use Alignment::*;

use crate::{
	chunk::{Lines, OpCode},
	stack::Stack,
	value::Value,
};

#[cfg(debug_assertions)]
use crate::debug::Repeat;

#[cfg(debug_assertions)]
pub(super) struct Disassembler {
	buf: UnsafeCell<String>,
	col: UnsafeCell<usize>,
}

// TODO - Replace messy crate::debug module with this

#[cfg(debug_assertions)]
impl Disassembler {
	// Column offsets for the output
	const ADDR: usize = 0;
	const LINE: usize = 12;
	const INSTR: usize = 14;
	const STACK: usize = 40;

	pub fn new() -> Self {
		Self {
			buf: UnsafeCell::new(String::new()),
			col: UnsafeCell::new(Self::ADDR),
		}
	}

	pub fn write_preamble(&self, offset: usize, lines: &Lines) {
		self.write_offset(offset);
		self.write_line(offset, lines);
	}

	fn write_offset(&self, offset: usize) {
		self.set_col(Self::ADDR);

		let data = format!("{:#06x}", offset);
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
			Some(prev) if prev == line => "|".to_string(),
			_ => line.to_string(),
		};

		self.write(data, Right);
	}

	pub fn write_opcode(&self, op: OpCode) {
		self.set_col(Self::INSTR);

		let data = format!("{:#04x} {:?}", op as u8, op);
		self.write(data, Left);
	}

	pub fn write_value(&self, value: Value) {
		let data = format!(" <{}>", value);
		self.write(data, Left);
	}

	pub fn write_stack(&self, stack: &Stack<Value>) {
		self.set_col(Self::STACK);

		let data = format!("{:?}", stack);
		self.write(data, Left);
	}

	pub fn flush(&self) {
		let buf = self.buf();
		println!("{}", buf);
		buf.clear();
	}

	fn write(&self, content: String, align: Alignment) {
		let buf = self.buf();
		match align {
			Left => {
				let pad = (self.col() as isize - buf.len() as isize).max(0) as usize;
				write!(buf, "{}{}", ' '.repeat(pad), content).unwrap();
			}
			Right => {
				let pad = (self.col() as isize
					- buf.len() as isize - content.len() as isize)
					.max(0) as usize;

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
	fn col(&self) -> usize {
		unsafe { *self.col.get() }
	}

	#[inline]
	fn set_col(&self, col: usize) {
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
	#[inline(always)] pub fn write_preamble(&self, _: usize, _: &Lines) {}
	#[inline(always)] pub fn write_opcode(&self, _: OpCode) {}
	#[inline(always)] pub fn write_value(&self, _: Value) {}
	#[inline(always)] pub fn write_stack(&self, _: &Stack<Value>) {}
	#[inline(always)] pub fn flush(&self) {}
}
