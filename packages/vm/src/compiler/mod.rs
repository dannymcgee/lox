use std::io::{self, Write};

use gramatika::{Parse, ParseStreamer};

use crate::chunk::Chunk;

use self::lexer::Stream;

mod lexer;

pub fn compile(src: String) -> anyhow::Result<Chunk> {
	let chunk = Stream::from(src).parse::<Chunk>()?;
	Ok(chunk)
}

impl<'a> Parse<'a> for Chunk {
	type Stream = Stream<'a>;

	fn parse(input: &mut Self::Stream) -> gramatika::Result<'a, Self> {
		let chunk = Chunk::new();

		let stdout = io::stdout();
		let mut stdout = stdout.lock();

		for token in input {
			writeln!(stdout, "{:?}", token).unwrap();
		}

		writeln!(stdout).unwrap();
		stdout.flush().unwrap();
		drop(stdout);

		Ok(chunk)
	}
}
