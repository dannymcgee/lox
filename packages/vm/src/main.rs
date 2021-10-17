use chunk::{Chunk, OpCode};

#[macro_use]
extern crate lazy_static;

mod chunk;
mod debug;
mod stack;
mod value;
mod vector;
mod vm;

fn main() -> vm::Result {
	let mut chunk = Chunk::new();
	chunk.write_const(1.2, 123);
	chunk.write_instr(OpCode::Negate, 123);
	chunk.write_instr(OpCode::Return, 123);

	chunk.write_const(420., 124);
	chunk.write_const(69., 124);
	chunk.write_instr(OpCode::Return, 124);

	vm::get().interpret(chunk)
}
