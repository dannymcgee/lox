#![feature(allocator_api)]
#![feature(stdio_locked)]

use repr::alloc;

#[macro_use]
extern crate gramatika;
extern crate lazy_static;

#[global_allocator]
static ALLOCATOR: alloc::Spy = alloc::Spy;

mod chunk;
mod cli;
mod compiler;
mod debug;
mod repl;
mod repr;
mod stack;
mod vector;
mod vm;

fn main() -> anyhow::Result<()> {
	let args = cli::args()?;

	if let Some(example) = args.example {
		vm::get().interpret(example).map(|_| ())?;
	} else {
		let term = cli::init()?;
		let mem_spy = alloc::Spy::enable_logging();

		repl::start()?;

		mem_spy.join().unwrap()?;
		term.join().unwrap()?;
	}

	Ok(())
}
