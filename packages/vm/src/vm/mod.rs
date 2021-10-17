use std::{cell::UnsafeCell, convert::TryFrom};

use crate::{
	chunk::{self, Chunk, JoinBytes, OpCode},
	stack::Stack,
	value::Value,
};

use self::debug::Disassembler;

mod debug;

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
	stack: UnsafeCell<Stack<Value>>,
	disasm: Disassembler,
}

// FIXME: This is definitely not sound
unsafe impl Send for VM {}
unsafe impl Sync for VM {}

macro_rules! binop {
	($self:ident, $stack:ident, $op:tt) => {{
		let rhs = $stack.pop().ok_or(Error::Runtime)?;
		$stack.mutate(|lhs| {
			$self.disasm.write_value(*lhs);
			$self.disasm.write_value(rhs);

			*lhs = *lhs $op rhs;
		});
	}}
}

impl VM {
	pub fn interpret(&self, chunk: Chunk) -> Result {
		unsafe {
			let ip = &mut *self.ip.get();
			*ip = Some(chunk.into_iter());
		}
		self.run()
	}

	fn run(&self) -> Result {
		use OpCode::*;

		let (ip, stack) = unsafe { (&mut *self.ip.get(), &mut *self.stack.get()) };
		assert!(
			ip.is_some(),
			"Called vm.run() with an unassigned instruction pointer"
		);

		let ip = ip.as_mut().unwrap();
		while let Some((offset, byte)) = ip.next() {
			self.disasm.write_preamble(offset, ip.lines());

			let op = OpCode::try_from(byte).map_err(|_| Error::Compile)?;
			self.disasm.write_opcode(op);

			#[rustfmt::skip]
			#[allow(clippy::assign_op_pattern)]
			match op {
				Constant | Constant16 | Constant24 => {
					let value = match op {
						Constant => ip.join_bytes(1),
						Constant16 => ip.join_bytes(2),
						Constant24 => ip.join_bytes(3),
						_ => unreachable!(),
					}
					.and_then(|handle| ip.read_const(handle))
					.ok_or(Error::Runtime)?;

					self.disasm.write_value(value);
					stack.push(value);
				}
				Add      => binop!(self, stack, +),
				Subtract => binop!(self, stack, -),
				Multiply => binop!(self, stack, *),
				Divide   => binop!(self, stack, /),
				Negate => {
					stack.mutate(|value| {
						self.disasm.write_value(*value);
						*value *= -1.;
					});
				}
				Return => {
					if let Some(value) = stack.pop() {
						self.disasm.write_value(value);
					}
				}
			};

			self.disasm.write_stack(stack);
			self.disasm.flush();
		}

		Ok(())
	}

	fn new() -> Self {
		VM {
			ip: UnsafeCell::new(None),
			stack: UnsafeCell::new(Stack::new()),
			disasm: Disassembler::new(),
		}
	}
}
