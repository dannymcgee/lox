#![feature(allocator_api)]

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
	cli::init();

	let _ = cli::args()?;
	let renderer = cli::render::start();
	let mem_spy = alloc::Spy::enable_logging();

	// {
	// 	let args = cli::args()?;
	// 	if let Some(example) = args.example {
	// 		vm::get().interpret(example).map(|_| ())
	// 	} else {
	// 		repl::start()
	// 	}?;
	// }

	mem_spy.join().unwrap();
	renderer.join().unwrap();

	Ok(())
}
