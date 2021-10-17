use std::{
	convert::TryFrom,
	fmt::{self, Alignment, Write},
};

use crate::chunk::{JoinBytes, Lines, OpCode, OpCodeError, Value};

pub trait DebugInstruction {
	fn debug_chunk<'a, I>(
		&mut self,
		bytes: &mut I,
		lines: &Lines,
		constants: &[Value],
	) -> fmt::Result
	where
		I: Iterator<Item = (usize, &'a u8)> + ExactSizeIterator + JoinBytes;

	fn debug_next_instr<I: JoinBytes>(
		&mut self,
		bytes: &mut I,
		lines: &Lines,
		constants: &[Value],
		offset: usize,
		byte: u8,
	) -> fmt::Result;

	fn print_offset(&mut self, offset: usize) -> fmt::Result;
	fn print_line_number(&mut self, lines: &Lines, offset: usize) -> fmt::Result;
	fn print_opcode(&mut self, op: OpCode) -> fmt::Result;
	fn print_opcode_and_value(
		&mut self,
		op: OpCode,
		handle: usize,
		value: Value,
	) -> fmt::Result;
}

impl<T: Write> DebugInstruction for T {
	fn debug_chunk<'a, I>(
		&mut self,
		bytes: &mut I,
		lines: &Lines,
		constants: &[Value],
	) -> fmt::Result
	where
		I: Iterator<Item = (usize, &'a u8)> + ExactSizeIterator + JoinBytes,
	{
		while let Some((offset, byte)) = bytes.next() {
			self.debug_next_instr(bytes, lines, constants, offset, *byte)?;

			// Insert newline if this isn't the last instruction
			if bytes.len() > 0 {
				writeln!(self)?;
			}
		}

		Ok(())
	}

	fn debug_next_instr<I: JoinBytes>(
		&mut self,
		bytes: &mut I,
		lines: &Lines,
		constants: &[Value],
		offset: usize,
		byte: u8,
	) -> fmt::Result {
		self.print_offset(offset)?;
		self.print_line_number(lines, offset)?;

		// Print the OpCode
		match OpCode::try_from(byte) {
			// For constants, we also print the index of the value in the pool,
			// followed by the value itself
			Ok(
				op @ OpCode::Constant | op @ OpCode::Constant16 | op @ OpCode::Constant24,
			) => {
				let handle = match op {
					OpCode::Constant => bytes.join_bytes(1),
					OpCode::Constant16 => bytes.join_bytes(2),
					OpCode::Constant24 => bytes.join_bytes(3),
					_ => unreachable!(),
				}
				.ok_or(fmt::Error)?;

				let value = constants[handle];

				self.print_opcode_and_value(op, handle, value)
			}
			Ok(op) => self.print_opcode(op),
			Err(OpCodeError(msg)) => write!(self, "<{}>", msg),
		}?;

		Ok(())
	}

	fn print_offset(&mut self, offset: usize) -> fmt::Result {
		write!(self, "{:04}  ", offset)
	}

	fn print_line_number(&mut self, lines: &Lines, offset: usize) -> fmt::Result {
		let line = lines.find_line(offset);
		let prev_line = if offset > 0 {
			Some(lines.find_line(offset - 1))
		} else {
			None
		};

		if prev_line.is_some() && prev_line.unwrap() == line {
			write!(self, "   | ")
		} else {
			write!(self, "{:>4} ", line)
		}
	}

	fn print_opcode(&mut self, op: OpCode) -> fmt::Result {
		write!(self, "{:?}", op)
	}

	fn print_opcode_and_value(
		&mut self,
		op: OpCode,
		handle: usize,
		value: Value,
	) -> fmt::Result {
		write!(self, "{:<16?}  [{}] '{}'", op, handle, value)
	}
}

pub fn print_aligned(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
	let len = text.len();
	let width = f.width().unwrap_or(len);
	let pad = (width - len).max(0);
	let fill = f.fill();

	match f.align() {
		None => write!(f, "{}", text),
		Some(Alignment::Center) => {
			// Err to the left if the space can't be evenly split
			let lw = (pad as f32 / 2.0).floor() as usize;
			let rw = pad - lw;

			let lp = fill.repeat(lw);
			let rp = fill.repeat(rw);

			write!(f, "{}{}{}", lp, text, rp)
		}
		Some(Alignment::Left) => write!(f, "{}{}", text, fill.repeat(pad)),
		Some(Alignment::Right) => write!(f, "{}{}", fill.repeat(pad), text),
	}
}

trait Repeat {
	fn repeat(&self, n: usize) -> String;
}

impl Repeat for char {
	fn repeat(&self, n: usize) -> String {
		let mut result = String::with_capacity(n);
		for _ in 0..n {
			result.push(*self);
		}
		result
	}
}
