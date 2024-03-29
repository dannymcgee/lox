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
	#[rustfmt::skip]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self {
			Self::Constant   => "CONSTANT",
			Self::Constant16 => "CONSTANT_16",
			Self::Constant24 => "CONSTANT_24",
			Self::True       => "TRUE",
			Self::False      => "FALSE",
			Self::Nil        => "NIL",
			Self::Add        => "ADD",
			Self::Subtract   => "SUBTRACT",
			Self::Multiply   => "MULTIPLY",
			Self::Divide     => "DIVIDE",
			Self::Negate     => "NEGATE",
			Self::Not        => "NOT",
			Self::Equal      => "EQUAL",
			Self::Greater    => "GREATER",
			Self::Less       => "LESS",
			Self::Return     => "RETURN",
		};
		debug::print_aligned(f, name)
	}
}
