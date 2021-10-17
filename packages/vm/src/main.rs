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
	chunk.write_instr(OpCode::Add, 124);
	chunk.write_instr(OpCode::Return, 124);

	// -((1.2 + 3.4) / 5.6)
	chunk.write_const(1.2, 125);
	chunk.write_const(3.4, 125);
	chunk.write_instr(OpCode::Add, 125);
	chunk.write_const(5.6, 125);
	chunk.write_instr(OpCode::Divide, 125);
	chunk.write_instr(OpCode::Negate, 125);
	chunk.write_instr(OpCode::Return, 125);

	vm::get().interpret(chunk)
}
