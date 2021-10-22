use std::{cell::UnsafeCell, convert::TryFrom};

use crate::{
	chunk::{self, JoinBytes, OpCode},
	compiler,
	repr::Value,
	stack::Stack,
};

use self::{debug::Disassembler, error::Error};

mod debug;
mod error;

// TODO: https://github.com/munificent/craftinginterpreters/blob/6c2ea6f7192910053a78832f0cc34ad56b17ce7c/book/a-virtual-machine.md?plain=1#L50
lazy_static! {
	static ref INSTANCE: VM = VM::new();
}

pub fn get() -> &'static VM {
	&INSTANCE
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
		let rhs_v = $stack.pop().unwrap();
		let rhs = match rhs_v {
			Value::Number(n) => Ok(n),
			other => Err(Error::Runtime(format!(
				"Binary operator `{}` not applicable to value `{}`",
				stringify!($op),
				other,
			))),
		}?;

		let mut result = Ok(());
		$stack.mutate(|lhs_v| {
			$self.disasm.write_value(lhs_v);
			$self.disasm.write_value(&rhs_v);

			match lhs_v {
				Value::Number(lhs) => *lhs_v = (*lhs $op rhs).into(),
				other => result = Err(Error::Runtime(format!(
					"Binary operator `{}` not applicable to value `{}`",
					stringify!($op),
					other,
				))),
			}
		});

		result?;
	}}
}

impl VM {
	fn new() -> Self {
		VM {
			ip: UnsafeCell::new(None),
			stack: UnsafeCell::new(Stack::new()),
			disasm: Disassembler::new(),
		}
	}

	pub fn interpret(&self, src: String) -> anyhow::Result<Option<Value>> {
		let chunk = compiler::compile(src)?;
		unsafe {
			let ip = &mut *self.ip.get();
			*ip = Some(chunk.into_iter());
		}

		self.disasm.write_header("chunk");
		self.run()
	}

	fn run(&self) -> anyhow::Result<Option<Value>> {
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
				Constant
				| Constant16
				| Constant24 => self.constant(op, ip, stack),
				Nil          => stack.push(Value::Nil),
				True         => stack.push(Value::Bool(true)),
				False        => stack.push(Value::Bool(false)),
				Add          => binop!(self, stack, +),
				Subtract     => binop!(self, stack, -),
				Multiply     => binop!(self, stack, *),
				Divide       => binop!(self, stack, /),
				Negate       => self.negate(stack)?,
				Not          => self.not(stack),
				Equal        => self.equal(stack),
				Greater      => binop!(self, stack, >),
				Less         => binop!(self, stack, <),
				Return       => self.return_(stack),
			};

			self.disasm.write_stack(stack);
			self.disasm.flush();
		}

		Ok(stack.pop())
	}

	fn constant(&self, op: OpCode, ip: &mut chunk::Consumable, stack: &mut Stack<Value>) {
		let value = match op {
			OpCode::Constant => ip.join_bytes(1),
			OpCode::Constant16 => ip.join_bytes(2),
			OpCode::Constant24 => ip.join_bytes(3),
			_ => unreachable!(),
		}
		.and_then(|handle| ip.read_const(handle))
		.expect("Error locating value in the pool");

		self.disasm.write_value(&value);
		stack.push(value);
	}

	fn negate(&self, stack: &mut Stack<Value>) -> anyhow::Result<()> {
		let mut result = Ok(());
		stack.mutate(|value| {
			self.disasm.write_value(value);
			match value {
				Value::Number(n) => *n *= -1.,
				other => {
					result = Err(Error::Runtime(format!(
						"Unary operator `-` not applicable to value: {}",
						other,
					)));
				}
			}
		});
		result?;

		Ok(())
	}

	fn not(&self, stack: &mut Stack<Value>) {
		stack.mutate(|value| *value = Value::Bool(value.is_falsy()))
	}

	fn equal(&self, stack: &mut Stack<Value>) {
		let rhs = stack.pop().unwrap();
		stack.mutate(|lhs| {
			*lhs = Value::Bool(*lhs == rhs);
		});
	}

	fn return_(&self, stack: &mut Stack<Value>) {
		if let Some(value) = stack.pop() {
			self.disasm.write_value(&value);
		}
	}
}
