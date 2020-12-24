#![allow(dead_code, unused_variables)]

use anyhow::Result;

mod chunk;
mod debug;

use chunk::*;

fn main() -> Result<()> {
	let mut chunk = Chunk::new("Test Chunk");

	chunk.write_instr(Op::Constant(1.2), 123)?;
	chunk.write_instr(Op::Return, 123)?;
	for idx in 0..=300 {
		chunk.write_instr(Op::Constant(idx.into()), 124)?;
	}

	println!("{:?}", chunk);

	Ok(())
}
