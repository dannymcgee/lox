use std::{
	convert::TryFrom,
	fmt::{self, Alignment},
	iter::Enumerate,
	slice::Iter,
};

use crate::chunk::OpCodeError;

use super::{Chunk, OpCode};

impl fmt::Debug for Chunk {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut stream = self.iter().enumerate().clone();

		while let Some((idx, byte)) = stream.next() {
			// Print the offset
			write!(f, "{:04}  ", idx)?;

			// Print the line number
			let line = self.lines.find_line(idx);
			let prev_line = if idx > 0 {
				Some(self.lines.find_line(idx - 1))
			} else {
				None
			};

			if prev_line.is_some() && prev_line.unwrap() == line {
				write!(f, "   | ")?;
			} else {
				write!(f, "{:>4} ", line)?;
			}

			// Print the OpCode
			match OpCode::try_from(*byte) {
				// For constants, we also print the index of the value in the pool,
				// followed by the value itself
				Ok(
					op @ OpCode::Constant
					| op @ OpCode::Constant16
					| op @ OpCode::Constant24,
				) => {
					let value_idx = match op {
						OpCode::Constant => get_value_index(&mut stream, 1),
						OpCode::Constant16 => get_value_index(&mut stream, 2),
						OpCode::Constant24 => get_value_index(&mut stream, 3),
						_ => unreachable!(),
					}
					.ok_or(fmt::Error)?;

					let value = self.constants[value_idx];

					write!(f, "{:<16?}  [{}] '{}'", op, value_idx, value)
				}
				Ok(op) => write!(f, "{:?}", op),
				Err(OpCodeError(msg)) => write!(f, "<{}>", msg),
			}?;

			// Insert newline if this isn't the last instruction
			if idx < self.len() - 1 {
				writeln!(f)?;
			}
		}

		Ok(())
	}
}

fn get_value_index(stream: &mut Enumerate<Iter<u8>>, byte_count: usize) -> Option<usize> {
	match byte_count {
		1 => {
			let (_, idx) = stream.next()?;
			Some(*idx as usize)
		}
		2 => {
			let (_, a) = stream.next()?;
			let (_, b) = stream.next()?;
			Some(u16::from_be_bytes([*a, *b]) as usize)
		}
		3 => {
			let (_, b) = stream.next()?;
			let (_, c) = stream.next()?;
			let (_, d) = stream.next()?;
			Some(u32::from_be_bytes([0, *b, *c, *d]) as usize)
		}
		_ => None,
	}
}

impl fmt::Debug for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self {
			Self::Return => "RETURN",
			Self::Constant => "CONSTANT",
			Self::Constant16 => "CONSTANT_16",
			Self::Constant24 => "CONSTANT_24",
		};
		print_aligned(f, name)
	}
}

fn print_aligned(f: &mut fmt::Formatter, text: &str) -> fmt::Result {
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
