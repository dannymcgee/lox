#![allow(dead_code)]

#[macro_use]
extern crate gramatika;
extern crate lazy_static;

mod chunk;
mod cli;
mod compiler;
mod debug;
mod repl;
mod stack;
mod value;
mod vector;
mod vm;

fn main() -> anyhow::Result<()> {
	let args = cli::args()?;

	if let Some(example) = args.example {
		vm::get().interpret(example)
	} else {
		repl::start()
	}
}
