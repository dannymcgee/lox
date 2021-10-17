use std::{
	convert::TryFrom,
	fmt::{self, Alignment},
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
			if idx > 0 && self.lines[idx] == self.lines[idx - 1] {
				write!(f, "   | ")?;
			} else {
				write!(f, "{:>4} ", self.lines[idx])?;
			}

			// Print the OpCode
			match OpCode::try_from(*byte) {
				// For constants, we also print the index of the value in the pool,
				// followed by the value itself
				Ok(op @ OpCode::Constant) => {
					let (_, value_idx) = stream.next().ok_or(fmt::Error)?;
					let value = self.constants[*value_idx as usize];

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

impl fmt::Debug for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self {
			Self::Constant => "CONSTANT",
			Self::Return => "RETURN",
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
