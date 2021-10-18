use std::{env, fs};

use itertools::Itertools;

mod parser;

#[derive(Debug)]
pub struct Args {
	pub example: Option<String>,
}

pub fn args() -> anyhow::Result<Args> {
	let raw = env::args().into_iter().skip(1).join("\n");
	let mut args = parser::parse(raw)?;

	if let Some(example) = args.example.as_mut() {
		let mut path = env::current_dir()?;
		path.extend(["packages", "spec", "src", "examples"]);
		path.push(format!("{}.lox", example));

		*example = fs::read_to_string(path)?;
	}

	Ok(args)
}
