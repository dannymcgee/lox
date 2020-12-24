use anyhow::Result;
use std::fmt::{self, Debug, Formatter};

use crate::chunk::{Chunk, Op};

impl Debug for Chunk {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "\n{:=^40}\n", format!(" {} ", self.name))?;

		let mut offset = 0;
		let mut line_idx = 0;
		let result = while offset < self.instructions.len() {
			let (size, content) = self.disassemble_at(offset).unwrap();

			write!(f, "{:04} ", offset)?;
			if line_idx > 0 && self.lines[line_idx] == self.lines[line_idx - 1] {
				write!(f, "   | ")?;
			} else {
				write!(f, "{:4} ", self.lines[offset])?;
			}
			write!(f, "{}", content)?;

			offset += size;
			line_idx += 1;
		};

		Ok(result)
	}
}

impl Chunk {
	fn disassemble_at(&self, idx: usize) -> Result<(usize, String)> {
		let byte = self.instructions[idx];
		match Op::from_byte(byte)? {
			Op::Constant(_) => {
				let idx_of_value = self.instructions[idx + 1];
				let value = &self.constants[idx_of_value as usize].0;
				let content =
					format!("CONSTANT {:>16} {}\n", format!("[{}]", idx_of_value), value);

				Ok((2, content))
			}
			Op::Constant16(_) => {
				let idx_slice = &self.instructions[idx + 1..=idx + 2];
				let idx_of_value = ((idx_slice[0] as u16) << 8) | idx_slice[1] as u16;
				let value = &self.constants[idx_of_value as usize].0;
				let content =
					format!("CONSTANT {:>16} {}\n", format!("[{}]", idx_of_value), value);

				Ok((3, content))
			}
			Op::Return => Ok((1, "RETURN\n".to_string())),
		}
	}
}

trait HexFormat {
	fn hex(self) -> String;
}
impl HexFormat for u8 {
	fn hex(self) -> String {
		format!("0x{:02X}", self)
	}
}
impl HexFormat for u16 {
	fn hex(self) -> String {
		format!("0x{:04X}", self)
	}
}
impl HexFormat for u32 {
	fn hex(self) -> String {
		format!("0x{:08X}", self)
	}
}
