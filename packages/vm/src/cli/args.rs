use core::fmt;
use std::{
	env, fs,
	sync::{Mutex, MutexGuard},
};

use bitflags::bitflags;
use gramatika::{Parse, ParseStreamer, Span, Spanned, SpannedError, Token as _};
use itertools::Itertools;

lazy_static! {
	static ref DEBUG_FLAGS: Mutex<DebugFlags> = Mutex::new(DebugFlags::NONE);
}

#[derive(Debug)]
pub struct Args {
	pub example: Option<String>,
	pub debug: DebugFlags,
}

bitflags! {
	pub struct DebugFlags: u8 {
		const NONE    = 0b000;
		const PARSE   = 0b001;
		const CODEGEN = 0b010;
		const EXEC    = 0b100;

		const COMPILE = Self::PARSE.bits | Self::CODEGEN.bits;
		const ALL = Self::COMPILE.bits | Self::EXEC.bits;
	}
}

pub fn args() -> anyhow::Result<Args> {
	let raw = env::args().into_iter().skip(1).join("\n");
	let (_src, mut args) = self::parse(raw)?;

	let mut flags = DEBUG_FLAGS.lock().unwrap();
	*flags = args.debug;
	drop(flags);

	if let Some(example) = args.example.as_mut() {
		let mut path = env::current_dir()?;
		path.extend(["packages", "spec", "src", "examples"]);
		path.push(format!("{}.lox", example));

		*example = fs::read_to_string(path)?;
	}

	Ok(args)
}

pub fn debug_flags<'a>() -> MutexGuard<'a, DebugFlags> {
	DEBUG_FLAGS.lock().unwrap()
}

fn parse(raw_args: String) -> gramatika::Result<'static, (String, Args)> {
	let mut parser = ParseStream::from(raw_args);
	let args = parser.parse::<Args>()?;
	let (src, _) = parser.into_inner();

	Ok((src, args))
}

type ParseStream<'a> = gramatika::ParseStream<'a, Token<'a>, Lexer<'a>>;

#[derive(Clone, Copy, DebugLispToken, Token, Lexer)]
pub enum Token<'a> {
	#[pattern = "--"]
	ArgStart(&'a str, Span),

	#[pattern = "[0-9.]+"]
	Number(&'a str, Span),

	#[pattern = "(true|false)"]
	Boolean(&'a str, Span),

	#[pattern = "="]
	Equal(&'a str, Span),

	#[pattern = "[_a-zA-Z][-_a-zA-Z0-9]+"]
	Word(&'a str, Span),
}

impl<'a> Parse<'a> for Args {
	type Stream = ParseStream<'a>;

	fn parse(input: &mut Self::Stream) -> gramatika::Result<'a, Self> {
		use TokenKind::*;

		let mut result = Args {
			example: None,
			debug: DebugFlags::NONE,
		};

		while !input.is_empty() {
			input.consume_kind(ArgStart)?;

			let key = input.consume_kind(Word)?;
			match key.lexeme() {
				"example" => {
					input.consume_kind(Equal)?;
					result.example = Some(input.consume_kind(Word)?.lexeme().into());
				}
				"debug" => {
					input.consume_kind(Equal)?;
					while !input.check_kind(ArgStart) {
						match input.next() {
							Some(Token::Word("parse", _)) => {
								result.debug |= DebugFlags::PARSE;
							}
							Some(Token::Word("codegen", _)) => {
								result.debug |= DebugFlags::CODEGEN;
							}
							Some(Token::Word("exec", _)) => {
								result.debug |= DebugFlags::EXEC;
							}
							Some(Token::Word("compile", _)) => {
								result.debug |= DebugFlags::COMPILE;
							}
							Some(Token::Boolean("true", _)) => {
								result.debug |= DebugFlags::ALL;
							}
							Some(Token::Boolean("false", _)) => {
								result.debug = DebugFlags::NONE;
							}
							Some(other) => {
								return Err(SpannedError {
									message: "Unrecognized debug argument".into(),
									source: input.source(),
									span: Some(other.span()),
								})
							}
							None => break,
						}
					}
				}
				_ => {
					return Err(SpannedError {
						source: input.source(),
						span: Some(key.span()),
						message: format!("Unknown argument '{}'", key.lexeme()),
					})
				}
			}
		}

		Ok(result)
	}
}

impl<'a> fmt::Debug for Token<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		<Self as gramatika::DebugLisp>::fmt(self, f, 0)
	}
}
