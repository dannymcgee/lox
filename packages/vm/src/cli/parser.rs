use core::fmt;

use gramatika::{Parse, ParseStreamer, Span, Spanned, SpannedError, Token as _};

use super::{Args, DebugFlags};

pub(super) fn parse(raw_args: String) -> gramatika::Result<'static, (String, Args)> {
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
