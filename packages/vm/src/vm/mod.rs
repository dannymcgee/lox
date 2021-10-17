use std::{cell::UnsafeCell, convert::TryFrom};

use crate::{
	chunk::{self, Chunk, JoinBytes, Lines, OpCode, Value},
	debug::DebugInstruction,
};

// TODO: https://github.com/munificent/craftinginterpreters/blob/6c2ea6f7192910053a78832f0cc34ad56b17ce7c/book/a-virtual-machine.md?plain=1#L50
lazy_static! {
	static ref INSTANCE: VM = VM::new();
}

pub fn get() -> &'static VM {
	&INSTANCE
}

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
	Compile,
	Runtime,
}

pub struct VM {
	ip: UnsafeCell<Option<chunk::Consumable>>,
	debugger: Debugger,
}

// FIXME: This is definitely not sound
unsafe impl Send for VM {}
unsafe impl Sync for VM {}

impl VM {
	pub fn interpret(&self, chunk: Chunk) -> Result {
		unsafe {
			let ip = &mut *self.ip.get();
			*ip = Some(chunk.into_iter());
		}

		self.run()
	}

	fn run(&self) -> Result {
		let ip = unsafe { &mut *self.ip.get() };
		assert!(
			ip.is_some(),
			"Called vm.run() with an unassigned instruction pointer"
		);

		let ip = ip.as_mut().unwrap();
		while let Some((offset, byte)) = ip.next() {
			self.debugger.write_preamble(offset, ip.lines());

			match OpCode::try_from(byte) {
				Ok(op @ OpCode::Return) => {
					self.debugger.print_opcode(op);
				}
				Ok(
					op @ OpCode::Constant
					| op @ OpCode::Constant16
					| op @ OpCode::Constant24,
				) => {
					let handle = match op {
						OpCode::Constant => ip.join_bytes(1),
						OpCode::Constant16 => ip.join_bytes(2),
						OpCode::Constant24 => ip.join_bytes(3),
						_ => unreachable!(),
					}
					.ok_or(Error::Runtime)?;
					let value = ip.read_const(handle).ok_or(Error::Runtime)?;

					self.debugger
						.print_opcode_and_value(op, handle, value);
				}
				_ => {}
			}
		}

		Ok(())
	}

	fn new() -> Self {
		VM {
			ip: UnsafeCell::new(None),
			debugger: Debugger::new(),
		}
	}
}

struct Debugger {
	#[cfg(debug_assertions)]
	buf: UnsafeCell<String>,
}

impl Debugger {
	#[cfg(debug_assertions)]
	pub fn new() -> Self {
		Self {
			buf: UnsafeCell::new(String::new()),
		}
	}

	#[cfg(debug_assertions)]
	pub fn write_preamble(&self, offset: usize, lines: &Lines) {
		let buf = unsafe { &mut *self.buf.get() };
		buf.print_offset(offset).unwrap();
		buf.print_line_number(lines, offset).unwrap();
	}

	#[cfg(debug_assertions)]
	pub fn print_opcode(&self, op: OpCode) {
		let buf = unsafe { &mut *self.buf.get() };
		buf.print_opcode(op).unwrap();

		self.flush();
	}

	#[cfg(debug_assertions)]
	pub fn print_opcode_and_value(&self, op: OpCode, handle: usize, value: Value) {
		let buf = unsafe { &mut *self.buf.get() };
		buf.print_opcode_and_value(op, handle, value)
			.unwrap();

		self.flush();
	}

	#[cfg(debug_assertions)]
	fn flush(&self) {
		let buf = unsafe { &mut *self.buf.get() };
		println!("{}", buf);
		buf.clear();
	}

	#[cfg(not(debug_assertions))]
	pub fn new() -> Self {
		Self
	}

	#[cfg(not(debug_assertions))]
	pub fn write_preamble(&self, _: usize, _: &Lines) {}

	#[cfg(not(debug_assertions))]
	pub fn print_opcode(&self, _: OpCode) {}

	#[cfg(not(debug_assertions))]
	pub fn print_opcode_and_value(&self, _: OpCode, _: usize, _: Value) {}

	#[cfg(not(debug_assertions))]
	fn flush(&self) {}
}
