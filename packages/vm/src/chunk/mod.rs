use std::{
	convert::TryFrom,
	ops::{Deref, DerefMut},
};

use num_derive::FromPrimitive;

mod debug;
mod into_iter;
mod join_bytes;
mod lines;

#[cfg(test)]
mod tests;

use crate::{
	value::Value,
	vector::{vector, Vector},
};

pub use self::{into_iter::Consumable, join_bytes::JoinBytes, lines::Lines};

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[rustfmt::skip]
pub enum OpCode {
	Constant   = 0x00,
	Constant16 = 0x01,
	Constant24 = 0x02,
	Add        = 0x10,
	Subtract   = 0x11,
	Multiply   = 0x12,
	Divide     = 0x13,
	Negate     = 0x14,
	Return     = 0xFF,
}

pub struct OpCodeError(pub String);

impl TryFrom<u8> for OpCode {
	type Error = OpCodeError;

	fn try_from(byte: u8) -> Result<Self, Self::Error> {
		match byte {
			0x00 => Ok(OpCode::Constant),
			0x01 => Ok(OpCode::Constant16),
			0x02 => Ok(OpCode::Constant24),
			0x10 => Ok(OpCode::Add),
			0x11 => Ok(OpCode::Subtract),
			0x12 => Ok(OpCode::Multiply),
			0x13 => Ok(OpCode::Divide),
			0x14 => Ok(OpCode::Negate),
			0xFF => Ok(OpCode::Return),
			_ => Err(OpCodeError(format!("UNKNOWN: {:#04x}", byte))),
		}
	}
}

pub struct Chunk {
	data: Vector<u8>,
	constants: Vector<Value>,
	lines: Lines,
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			data: vector![],
			constants: vector![],
			lines: Lines::new(),
		}
	}

	pub fn write_instr(&mut self, op: OpCode, line: usize) {
		self.write(op as u8, line);
	}

	pub fn write_const(&mut self, value: Value, line: usize) {
		let handle = self.add_constant(value);

		match handle {
			0..=255 => {
				self.write(OpCode::Constant as u8, line);
				self.write(handle as u8, line);
			}
			256..=65_535 => {
				self.write(OpCode::Constant16 as u8, line);
				let bytes = (handle as u16).to_be_bytes();
				self.extend(&bytes, line);
			}
			_ => {
				self.write(OpCode::Constant24 as u8, line);
				let [_, b, c, d] = (handle as u32).to_be_bytes();
				self.extend(&[b, c, d], line);
			}
		}
	}

	fn write(&mut self, byte: u8, line: usize) {
		self.data.push(byte);
		self.lines.add_byte(line, self.data.len() - 1);
	}

	fn extend(&mut self, bytes: &[u8], line: usize) {
		self.lines.add_byte(line, self.data.len());
		for byte in bytes.iter() {
			self.data.push(*byte);
		}
	}

	fn add_constant(&mut self, value: Value) -> usize {
		self.constants.push(value);
		self.constants.len() - 1
	}
}

impl Deref for Chunk {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&*self.data
	}
}

impl DerefMut for Chunk {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut *self.data
	}
}
