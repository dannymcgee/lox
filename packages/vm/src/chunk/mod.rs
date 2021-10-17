use std::{
	convert::TryFrom,
	ops::{Deref, DerefMut},
};

use num_derive::FromPrimitive;

mod debug;
mod value;
mod vector;

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! chunk {
	[] => {
		$crate::chunk::Chunk::new()
	};
	[$($byte:expr),+ $(,)?] => {{
		let mut chunk = $crate::chunk::Chunk::new();
		$(chunk.write($byte);)*
		chunk
	}};
}

pub use chunk;

use self::value::Value;
pub use self::vector::{vector, Vector};

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[rustfmt::skip]
pub enum OpCode {
	Constant = 0x00,
	Return   = 0x01,
}

pub struct OpCodeError(pub String);

impl TryFrom<u8> for OpCode {
	type Error = OpCodeError;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(OpCode::Constant),
			0x01 => Ok(OpCode::Return),
			_ => Err(OpCodeError(format!("UNKNOWN: {:#04x}", value))),
		}
	}
}

pub struct Chunk {
	data: Vector<u8>,
	constants: Vector<Value>,
	lines: Vector<usize>,
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			data: vector![],
			constants: vector![],
			lines: vector![],
		}
	}

	pub fn write(&mut self, byte: u8, line: usize) {
		self.data.push(byte);
		self.lines.push(line);
		assert_eq!(self.data.len(), self.lines.len());
	}

	pub fn add_constant(&mut self, value: Value) -> usize {
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
