use std::fmt;

use crate::debug::{self, DebugInstruction};

use super::{Chunk, OpCode};

impl fmt::Debug for Chunk {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut bytes = self.iter().enumerate();
		let lines = &self.lines;
		let constants = &self.constants;

		f.debug_chunk(&mut bytes, lines, constants)
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
		debug::print_aligned(f, name)
	}
}
