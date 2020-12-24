use anyhow::{Error, Result};

pub enum Op {
	Constant(f64),
	Constant16(f64),
	Return,
}

#[rustfmt::skip]
impl Op {
	pub fn from_byte(byte: u8) -> Result<Self> {
		match byte {
			00 => Ok(Op::Constant(0.0)),
			99 => Ok(Op::Constant16(0.0)),
			01 => Ok(Op::Return),
			_  => Err(Error::msg("Could not convert byte to OpCode")),
		}
	}
	fn as_byte(&self) -> u8 {
		match self {
			Op::Constant(_)   => 00,
			Op::Constant16(_) => 99,
			Op::Return        => 01,
		}
	}
}

pub struct Value(pub f64);

#[derive(Default)]
pub struct Chunk {
	pub name: &'static str,
	pub instructions: Vec<u8>,
	pub lines: Vec<usize>,
	pub constants: Vec<Value>,
}

impl Chunk {
	pub fn new(name: &'static str) -> Self {
		Chunk {
			name,
			..Default::default()
		}
	}
	pub fn write_instr(&mut self, op_code: Op, line: usize) -> Result<()> {
		match op_code {
			Op::Constant(val) => {
				let idx = self.add_constant(val)?;

				if idx > 255 {
					self.instructions.push(Op::Constant16(val).as_byte());
					self.instructions.push((idx >> 8) as u8);
					self.instructions.push((idx & 0xff) as u8);
				} else {
					self.instructions.push(op_code.as_byte());
					self.instructions.push(idx as u8);
				}
			}
			_ => self.instructions.push(op_code.as_byte()),
		}
		self.lines.push(line);

		Ok(())
	}
	fn add_constant(&mut self, value: f64) -> Result<u16> {
		self.constants.push(Value(value));
		let idx = self.constants.len() - 1;
		if idx > std::u16::MAX.into() {
			Err(Error::msg("Too many constants!"))
		} else {
			Ok(idx as u16)
		}
	}
}
