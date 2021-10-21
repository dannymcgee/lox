use gramatika::{Parse, ParseStreamer, Result, Span};
use macro_utils::trace;

use crate::{
	chunk::{Chunk, OpCode},
	compiler::pratt::PrattParser,
	value::Value,
	*,
};

use self::lexer::Stream;

#[macro_use]
mod lexer;

#[cfg(debug_assertions)]
mod debug;

mod pratt;
mod prec;

pub fn compile(src: String) -> anyhow::Result<Chunk> {
	debug::write_header("chunk");

	let mut stream = Stream::from(src);
	let mut chunk = stream.parse::<Chunk>()?;

	let (src, _) = stream.into_inner();
	chunk.set_source(src);

	Ok(chunk)
}

impl<'a> Parse<'a> for Chunk
where 'a: 'static
{
	type Stream = Stream<'a>;

	fn parse(input: &mut Self::Stream) -> Result<'a, Self> {
		let mut chunk = Chunk::new();

		while !input.is_empty() {
			chunk.expression(input)?;
		}

		Ok(chunk)
	}
}

impl Chunk {
	#[trace(debug::codegen_instr)]
	fn emit_instr(&mut self, op: OpCode, span: Span) {
		self.write_instr(op, span.start.line + 1);
	}

	#[trace(debug::codegen_pair)]
	fn emit_pair(&mut self, pair: (OpCode, OpCode), span: Span) {
		let (a, b) = pair;
		self.extend(&[a as u8, b as u8], span.start.line + 1);
	}

	#[trace(debug::codegen_const)]
	fn emit_const(&mut self, value: Value, span: Span) {
		self.write_const(value, span.start.line + 1);
	}
}

#[rustfmt::skip]
#[cfg(not(debug_assertions))]
mod debug {
	use gramatika::Span;

	use super::{
		chunk::OpCode,
		lexer::{Stream, Token},
		pratt::HashToken,
		prec::Prec,
		value::Value,
	};

	pub enum RuleType {
		Prefix,
		Infix,
	}

	// Noop module for release target -- with empty function bodies and #[inline(always)],
	// calls to these should be eliminated from the output entirely

	#[inline(always)] pub(super) fn write_header(_: &str) {}
	#[inline(always)] pub(super) fn entry(_: &'static str, _: &mut Stream) {}
	#[inline(always)] pub(super) fn precedence(_: &'static str, _: &mut Stream, _: Prec) {}
	#[inline(always)] pub(super) fn get_rule(_: &'static str, _: HashToken) {}
	#[inline(always)] pub(super) fn set_rule_type(_: RuleType) {}
	#[inline(always)] pub(super) fn parse_fn(_: &'static str, _: &mut Stream) {}
	#[inline(always)] pub(super) fn codegen_instr(_: &'static str, _: OpCode, _: Span) {}
	#[inline(always)] pub(super) fn codegen_const(_: &'static str, _: Value, _: Span) {}
}
