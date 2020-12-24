#![allow(dead_code, unused_variables)]

use anyhow::Result;

mod chunk;
mod debug;

use chunk::*;

fn main() -> Result<()> {
	let mut chunk = Chunk::new("Test Chunk");

	chunk.write_instr(Op::Constant(1.2), 123)?;
	chunk.write_instr(Op::Return, 123)?;

	println!("{:?}", chunk);

	Ok(())
}
